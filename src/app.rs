// SPDX-License-Identifier: MPL-2.0

use crate::config::Config;
use crate::fl;
use cosmic::app::{context_drawer, Core, Task};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
//use image::ImageReader;
//use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Subscription};
use cosmic::iced_core;
//use cosmic::iced_wgpu::graphics::text;
use cosmic::iced_widget::image::{self, Handle};
use cosmic::widget::{self, icon, menu, nav_bar};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};
use futures_util::SinkExt;
use std::collections::HashMap;
use std::fmt::Debug;
mod scan;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

#[derive(Debug, Clone)]
struct Song {
    title: String,
    artist: String,
    album: String,
    year: String,
}
impl Default for Song {
    fn default() -> Self {
        Self {
            title: "".into(),
            artist: "".into(),
            album: "".into(),
            year: "".into(),
        }
    }
}
struct Album {
    title: String,
    artist: String,
    year: String,
    songs: Vec<Song>,
}
impl Default for Album {
    fn default() -> Self {
        Self {
            title: "".into(),
            artist: "".into(),
            year: "2020".into(),
            songs: vec![],
        }
    }
}
/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    // Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    // Configuration data that persists between application runs.
    config: Config,

    active_page: Page,
    library: Vec<Song>,
    albums: Vec<Album>,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    OpenRepositoryUrl,
    SubscriptionChannel,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    LaunchUrl(String),
}

fn load_music() -> (Vec<Song>, Vec<Album>) {
    let music_dir = "/home/aidanw/Music/";
    let songs = scan::scan_music_files(music_dir);
    let mut library: Vec<Song> = Vec::new();
    let mut albums: HashMap<String, Album> = HashMap::new();

    for file in songs {
        //println!("{:?}", file);
        if let Some((title, artist, album_name)) = scan::extract_metadata(&file) {
            let song = Song {
                title,
                artist: artist.clone(),
                album: album_name.clone(),
                year: "2020".into(),
            };
            library.push(song);

            // Create or update album
            let album_entry = albums.entry(album_name.clone()).or_insert_with(|| Album {
                title: album_name.clone(),
                artist: artist.clone(),
                year: "2020".into(),
                songs: Vec::new(),
            });
            album_entry.songs.push(library.last().unwrap().clone());

            // Only try to extract artwork if we haven't found it yet for this album
            //if album_entry.artwork.is_none() {
            //    if let Some(artwork) = scan::extract_artwork(&file) {
            //        // Convert DynamicImage to memory buffer
            //        let cover = ImageReader::open("`")
            //        album_entry.artwork = Some(Handle::from_memory(buffer));
            //    }
            //}
        }
    }

    (library, albums.into_values().collect())
}
/// Create a COSMIC application from the app model
impl Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.SavNov.Rivet";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        // Create a nav bar with three page items.
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text("Albums")
            .data::<Page>(Page::Albums)
            .icon(icon::from_name("applications-science-symbolic"))
            .activate();

        nav.insert()
            .text("Artists")
            .data::<Page>(Page::Artists)
            .icon(icon::from_name("applications-system-symbolic"));

        nav.insert()
            .text("Songs")
            .data::<Page>(Page::Songs)
            .icon(icon::from_name("applications-games-symbolic"));

        // Load music library
        let (library, albums) = load_music();

        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            nav,
            key_binds: HashMap::new(),
            active_page: Page::Albums,
            library,
            albums,
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
        };

        // Create a startup command that sets the window title.
        let command = app.update_title();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::context_drawer(
                self.about(),
                Message::ToggleContextPage(ContextPage::About),
            )
            .title(fl!("about")),
        })
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<Self::Message> {
        let content = match self.active_page {
            Page::Albums => self.view_albums(),
            Page::Artists => self.view_artists(),
            Page::Songs => self.view_songs(&self.library),
        };
        widget::column().spacing(20).push(content).into()
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run_with_id(
                std::any::TypeId::of::<MySubscription>(),
                cosmic::iced::stream::channel(4, move |mut channel| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;

                    futures_util::future::pending().await
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::OpenRepositoryUrl => {
                _ = open::that_detached(REPOSITORY);
            }

            Message::SubscriptionChannel => {
                // For example purposes only.
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },
        }
        Task::none()
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);

        if let Some(&page) = self.nav.data::<Page>(id) {
            self.active_page = page;
        }

        self.update_title()
    }
}

impl AppModel {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = widget::text::title3(fl!("app-title"));

        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .push(
                widget::button::link(fl!(
                    "git-description",
                    hash = short_hash.as_str(),
                    date = date
                ))
                .on_press(Message::LaunchUrl(format!("{REPOSITORY}/commits/{hash}")))
                .padding(0),
            )
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<Message> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }

    fn view_albums(&self) -> Element<Message> {
        let mut grid = widget::column();

        // Create a row for every 3 albums
        let mut current_column = widget::column().spacing(20);
        let mut current_row = widget::row::with_capacity(3).spacing(10);
        let mut column = 0;
        let mut row = 0;

        for album in self.albums.iter() {
            let album_widget = widget::column().spacing(10);

            // Add artwork if available, otherwise use placeholder
            //let artwork_widget = if let artwork = &album.artwork {
            //    widget::image(artwork.clone()).width(180).height(180)
            //} else {
            //    //widget::image(ImageFormat::from_name("audio-x-generic"))
            //    //    .width(180)
            //    //    .height(180)
            //    continue;
            //};

            let album_widget = album_widget
                //.push(artwork_widget)
                .push(widget::text(&album.title).size(16))
                .push(widget::text(&album.artist).size(14));

            current_column = current_column.push(album_widget);
            column += 1;

            if column == 3 {
                current_row = current_row.push(widget::row().spacing(45).push(current_column));
                println!("{}", row);
                current_column = widget::column().spacing(20);
                column = 0;
                row += 1;
                if row == 3 {
                    println!("{}", row);
                    grid = grid.push(widget::column().push(current_row).push(widget::text("2!")));
                    current_row = widget::row().spacing(45);
                    row = 0;
                }
            }
        }

        // Push any remaining albums
        if column > 0 {
            //grid = grid.push(current_row);
        }

        grid.into()
    }

    fn view_artists(&self) -> Element<Message> {
        widget::text("Artists page, WIP!").into()
    }

    fn view_songs(&self, songs: &Vec<Song>) -> Element<Message> {
        let songs = songs;
        let mut column = widget::column()
            .spacing(10)
            .push(widget::text::title1("Songs"));

        for song in songs.iter() {
            column = column
                .push(widget::text(format!("{}", song.title,)))
                .push(widget::text(format!("{}", song.artist)))
                .push(widget::row().spacing(10));
            println!("{}", song.title);
        }
        column.into()
    }
}

/// The page to display in the application.
#[derive(Debug, Clone, Copy)]
pub enum Page {
    Albums,
    Artists,
    Songs,
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}
