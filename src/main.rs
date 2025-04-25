use iced::{
    Alignment, Command, Element, Length, Sandbox, Settings, Theme,
    widget::{Button, Column, Row, button, column, container, row, scrollable, text},
};
#[derive(Debug, Clone, Copy)]
enum Menu {
    Home,
    Settings,
    About,
}

#[derive(Debug, Clone)]
enum Message {
    MenuSelected(Menu),
    ScrolledDown,
    ScrolledUp,
}

struct SidePanelApp {
    current_menu: Menu,
    //home_button: button::State,
    //settings_button: button::State,
    //about_button: button::State,
}

impl Sandbox for SidePanelApp {
    type Message = Message;

    fn new() -> SidePanelApp {
        Self {
            current_menu: Menu::Home,
            //home_button: button::State::new(),
            //settings_button: button::State::new(),
            //about_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Side Panel Example")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MenuSelected(menu) => {
                self.current_menu = menu;
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let menu_buttons = Column::new()
            .spacing(10)
            .push(Button::new(text("Home")).on_press(Message::MenuSelected(Menu::Home)))
            .push(Button::new(text("Settings")).on_press(Message::MenuSelected(Menu::Settings)))
            .push(Button::new(text("About")).on_press(Message::MenuSelected(Menu::About)));
        let main_content = match self.current_menu {
            Menu::Home => view_home(),
            Menu::Settings => view_settings(),
            Menu::About => view_about(),
        };

        Row::new()
            .push(container(menu_buttons).width(150).padding(10))
            .push(container(main_content).padding(20).center_y())
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinFrappe
    }
}

fn view_home() -> Element<'static, Message> {
    let content = text("Welcome Home!");
    Row::new()
        .push(container(content).padding(20).center_y())
        .into()
}

fn view_settings() -> Element<'static, Message> {
    let content = text("Settings!");
    Row::new()
        .push(container(content).padding(20).center_y())
        .into()
}
fn view_about() -> Element<'static, Message> {
    let content = text("About This App...");
    Row::new()
        .push(container(content).padding(20).center_y())
        .into()
}
fn main() -> iced::Result {
    SidePanelApp::run(Settings::default())
}
