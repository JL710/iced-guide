// ANCHOR: all
use iced::{widget, Sandbox};

// ANCHOR: message_enum
#[derive(Debug, Clone, Copy)]
enum Message {
    IncreaseCounter,
    DecreaseCounter,
}
// ANCHOR_END: message_enum

// ANCHOR: counter_struct
struct Counter {
    number: i32,
}
// ANCHOR_END: counter_struct

// ANCHOR: sandbox_for_counter
impl Sandbox for Counter {
    // ANCHOR: counter_message_type
    type Message = Message;
    // ANCHOR_END: counter_message_type

    // ANCHOR: new_and_title
    fn new() -> Self {
        Self { number: 0 }
    }

    fn title(&self) -> String {
        String::from("Window Title")
    }
    // ANCHOR_END: new_and_title

    // ANCHOR: update
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncreaseCounter => self.number += 1,
            Message::DecreaseCounter => self.number -= 1,
        }
    }
    // ANCHOR_END: update

    // ANCHOR: view
    fn view(&self) -> iced::Element<'_, Self::Message> {
        // ANCHOR: column
        widget::column![
            // ANCHOR: view_text
            widget::text(self.number),
            // ANCHOR_END: view_text
            // ANCHOR: view_buttons
            widget::button("Increase").on_press(Message::IncreaseCounter),
            widget::button("Decrease").on_press(Message::DecreaseCounter)
            // ANCHOR_END: view_buttons
        ]
        // ANCHOR_END: column
        .into()
    }
    // ANCHOR_END: view
}
// ANCHOR_END: sandbox_for_counter

// ANCHOR: main
fn main() {
    Counter::run(iced::Settings::default()).unwrap();
}
// ANCHOR_END: main
// ANCHOR_END: all
