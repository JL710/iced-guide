// ANCHOR: all
use iced::{widget, Sandbox};

// ANCHOR: counter_struct
struct Counter {
    // This will be our state of the counter app
    // a.k.a the current count value
    count: i32,
}
// ANCHOR_END: counter_struct

// ANCHOR: message_enum
#[derive(Debug, Clone, Copy)]
enum Message {
    // Emitted when the increment ("+") button is pressed
    IncrementCount,
    // Emitted when decrement ("-") button is pressed
    DecrementCount,
}
// ANCHOR_END: message_enum

// ANCHOR: sandbox_for_counter
// Implement Sandbox for our Counter
impl Sandbox for Counter {

    // ANCHOR: counter_message_type
    // alias our Message enum with the
    // Sandbox's Message type
    type Message = Message;
    // ANCHOR_END: counter_message_type

    // ANCHOR: new
    fn new() -> Self {
        // initialize the counter struct
        // with count value as 0.
        Self { count: 0 }
    }
    // ANCHOR_END: new

    // ANCHOR: title
    fn title(&self) -> String {
        //define the title for our app
        String::from("Counter App")
    }
    // ANCHOR_END: title

    // ANCHOR: update
    fn update(&mut self, message: Self::Message) {
        // handle emitted messages
        match message {
            Message::IncreaseCounter => self.count += 1,
            Message::DecreaseCounter => self.count -= 1,
        }
    }
    // ANCHOR_END: update

    // ANCHOR: view
    fn view(&self) -> iced::Element<'_, Self::Message> {
        // create the View Logic (UI)
        // ANCHOR: row
        let rw = widget::row![
            // ANCHOR: view_buttons
            widget::button("-").on_press(Message::DecrementCount),
            // ANCHOR: view_text
            widget::text(self.count),
            // ANCHOR_END: view_text
            widget::button("+").on_press(Message::IncrementCount)
            // ANCHOR_END: view_buttons
        ]
        // ANCHOR_END: row
        widget::container(rw)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .Height(iced::Length::Fill)
            .into()
    }
    // ANCHOR_END: view
}
// ANCHOR_END: sandbox_for_counter

// ANCHOR: main
fn main() -> Result<(), iced::Error> {
    // run the app from main function
    Counter::run(iced::Settings::default())
}
// ANCHOR_END: main
// ANCHOR_END: all
