// ANCHOR: all
use iced::{Element, Theme, widget::pick_list};

// ANCHOR: main
fn main() -> iced::Result {
    iced::application("Themes", App::update, App::view)
        .theme(App::theme)
        .run()
}
// ANCHOR_END: main

#[derive(Default)]
struct App {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
}

impl App {
    // ANCHOR: theme_fn
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
    // ANCHOR_END: theme_fn

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
        };
    }

    fn view(&self) -> Element<'_, Message> {
        pick_list(Theme::ALL, Some(self.theme.clone()), Message::ThemeChanged).into()
    }
}
// ANCHOR_END: all
