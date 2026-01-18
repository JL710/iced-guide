// ANCHOR: all
use iced::widget;

// ANCHOR: message_enum
#[derive(Debug, Clone)]
enum Message {
    Event(iced::event::Event),
}
// ANCHOR_END: message_enum

// ANCHOR: update
fn update(state: &mut u32, message: Message) {
    // handle emitted messages
    match message {
        Message::Event(event) => {
            if let iced::event::Event::Keyboard(iced::keyboard::Event::KeyReleased {
                key, ..
            }) = event
            {
                println!("Key {key:?} was pressed");
                *state += 1;
            }
        }
    }
}
// ANCHOR_END: update

// ANCHOR: view
fn view(state: &u32) -> iced::Element<'_, Message> {
    widget::text!("Keys pressed: {}", state).into()
}
// ANCHOR_END: view

// ANCHOR: main
fn main() -> iced::Result {
    // run the app from main function
    iced::application(|| 0, update, view)
        .subscription(|_state| iced::event::listen().map(Message::Event))
        .run()
}
// ANCHOR_END: main
// ANCHOR_END: all
