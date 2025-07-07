// ANCHOR: all
use iced::widget;

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

// ANCHOR: implement_counter
// Implement our Counter
impl Counter {
    // ANCHOR: new
    fn new() -> Self {
        // initialize the counter struct
        // with count value as 0.
        Self { count: 0 }
    }
    // ANCHOR_END: new

    // ANCHOR: update
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        // handle emitted messages
        match message {
            Message::IncrementCount => self.count += 1,
            Message::DecrementCount => self.count -= 1,
        }
        iced::Task::none()
    }
    // ANCHOR_END: update

    // ANCHOR: view
    fn view(&self) -> iced::Element<'_, Message> {
        // create the View Logic (UI)
        // ANCHOR: row
        let row = widget::row![
            // ANCHOR: view_buttons
            widget::button("-").on_press(Message::DecrementCount),
            // ANCHOR: view_text
            widget::text!("Count: {}", self.count),
            // ANCHOR_END: view_text
            widget::button("+").on_press(Message::IncrementCount) // ANCHOR_END: view_buttons
        ]
        .spacing(10);
        // ANCHOR_END: row
        widget::container(row)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
    // ANCHOR_END: view
}
// ANCHOR_END: implement_counter

// ANCHOR: main
fn main() -> Result<(), iced::Error> {
    // run the app from main function
    iced::application("Counter Example", Counter::update, Counter::view)
        .run_with(|| (Counter::new(), iced::Task::none()))
}
// ANCHOR_END: main
// ANCHOR_END: all
