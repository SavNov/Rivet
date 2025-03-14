use cosmic::{
    app::{App, WindowSettings},
    elements::{Button, Column, Label},
    widget::{self, Element},
    window, Color,
};

#[derive(Default)]
struct HelloApp {
    counter: i32,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl App for HelloApp {
    type Message = Message;

    fn title(&self) -> String {
        "Hello Cosmic".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed => {
                self.counter += 1;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let label = Label::new(format!("Count: {}", self.counter));

        let button = Button::new("Click me!")
            .on_press(Message::ButtonPressed);

        Column::new()
            .push(label)
            .push(button)
            .into()
    }

    fn theme(&self) -> cosmic::Theme {
        cosmic::Theme::Dark
    }
}

fn main() -> cosmic::Result<()> {
    window::run::<HelloApp>(WindowSettings::default())
}
