// ANCHOR: all
use iced::{
    Element, Theme, border, color,
    widget::{button, row},
};

fn main() -> iced::Result {
    iced::run("Styling", App::update, App::view)
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
                .style(button_background) // ANCHOR_END: style_method
        ]
        .spacing(10)
        .into()
    }
}

// ANCHOR: button_background
// This style was backported from 0.14, where it's available built in.
fn button_background(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = button::Style {
        background: Some(palette.background.base.color.into()),
        text_color: palette.background.base.text,
        border: border::rounded(2),
        ..Default::default()
    };

    match status {
        button::Status::Active => base,
        button::Status::Pressed => button::Style {
            background: Some(palette.background.strong.color.into()),
            ..base
        },
        button::Status::Hovered => button::Style {
            background: Some(palette.background.weak.color.into()),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: base
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}
// ANCHOR_END: button_background
// ANCHOR_END: all
