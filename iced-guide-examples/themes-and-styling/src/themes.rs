// ANCHOR: all
use iced::{Element, Theme, widget::pick_list};

// ANCHOR: main
fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(App::theme)
        .run()
}
// ANCHOR_END: main

#[derive(Default)]
struct App {
    theme: Option<Theme>,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
}

impl App {
    // ANCHOR: theme_fn
    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
    // ANCHOR_END: theme_fn

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = Some(theme),
        };
    }

    fn view(&self) -> Element<'_, Message> {
        pick_list(Theme::ALL, self.theme.clone(), Message::ThemeChanged)
            .placeholder("Choose a theme...")
            .into()
    }
}
// ANCHOR_END: all
