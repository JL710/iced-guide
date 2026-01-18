// ANCHOR: all
use iced::{
    Element, Theme, border, color,
    widget::{button, row},
};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {
    Noop,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Noop => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        row![
            // ANCHOR: builtin_style
            button("Built-in 'danger' style")
                .on_press(Message::Noop)
                .style(button::danger),
            // ANCHOR_END: builtin_style
            // ANCHOR: inline_style
            button("User defined inline style")
                .on_press(Message::Noop)
                .style(|_, _| button::Style {
                    background: Some(color!(0x1e1e2e).into()),
                    text_color: color!(0xc0ffee),
                    border: border::rounded(10),
                    ..Default::default()
                }),
            // ANCHOR_END: inline_style
            // ANCHOR: style_method
            button("User defined style method")
                .on_press(Message::Noop)
                .style(button_danger_text)
            // ANCHOR_END: style_method
        ]
        .spacing(10)
        .into()
    }
}

// ANCHOR: button_danger_text
// This is a *slightly* modified version of button::text
fn button_danger_text(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let base = button::Style {
        text_color: palette.danger.base.color,
        ..button::Style::default()
    };

    match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            text_color: palette.danger.base.color.scale_alpha(0.8),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: palette.danger.base.color.scale_alpha(0.5),
            ..base
        }
    }
}
// ANCHOR_END: button_danger_text
// ANCHOR_END: all
