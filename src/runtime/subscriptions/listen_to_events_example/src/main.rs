// ANCHOR: all
use iced::widget;

// ANCHOR: message_enum
#[derive(Debug, Clone)]
enum Message {
    Event(iced::event::Event),
}
// ANCHOR_END: message_enum

// ANCHOR: update
fn update(_state: &mut u8, message: Message) -> iced::Task<Message> {
    // handle emitted messages
    match message {
        Message::Event(event) => match event {
            iced::event::Event::Keyboard(keyboard_event) => match keyboard_event {
                iced::keyboard::Event::KeyReleased { key, .. } => {
                    println!("Key {:?} was pressed", key);
                }
                _ => {}
            },
            _ => {}
        },
    }
    iced::Task::none()
}
// ANCHOR_END: update

// ANCHOR: view
fn view(_state: &u8) -> iced::Element<'_, Message> {
    widget::text("Event Example").into()
}
// ANCHOR_END: view

// ANCHOR: main
fn main() -> Result<(), iced::Error> {
    // run the app from main function
    iced::application("Event example", update, view)
        .subscription(|_state| iced::event::listen().map(Message::Event))
        .run()
}
// ANCHOR_END: main
// ANCHOR_END: all
