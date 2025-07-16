// ANCHOR: all

use iced::widget::{button, center, text};
use iced::{Element, Task};

use std::time::Duration;

fn main() -> iced::Result {
    iced::application("Find the meaning of life", App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    FindTheMeaningOfLife,
    TheMeaningOfLife(Option<MeaningOfLife>),
}

#[derive(Default, Debug)]
enum State {
    #[default]
    Unknown,
    Searching,
    Found(MeaningOfLife),
    NotFound,
}

#[derive(Debug, Default)]
struct App {
    state: State,
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        let main: Element<_> = match &self.state {
            State::Unknown => {
                button("Find the meaning of life.")
                    .on_press(Message::FindTheMeaningOfLife)
                    .into()
            },
            State::Searching => text("Searching...").into(),
            State::Found(MeaningOfLife(meaning)) => text(meaning).into(),
            State::NotFound => text("Could not find the meaning of life.").into(),
        };

        center(main).into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FindTheMeaningOfLife => {
                self.state = State::Searching;

                Task::perform(
                    meaning_of_life(), 
                    Message::TheMeaningOfLife
                )
            }
            Message::TheMeaningOfLife(meaning) => {
                self.state = match meaning {
                    Some(meaning) => State::Found(meaning),
                    None => State::NotFound,
                };

                Task::none()
            }
        }
    }
}

// ANCHOR: oneshot
#[derive(Debug, Clone)]
struct MeaningOfLife(String);

async fn meaning_of_life() -> Option<MeaningOfLife> {
    use iced::futures::channel;

    // Create a oneshot channel for the thread to send its result.
    let (result_tx, result_rx) = channel::oneshot::channel();

    // Spawn a thread so that our calculation doesn't block the main thread.
    std::thread::spawn(|| {
        let result = calculate_meaning();

        result_tx
            .send(result)
            .expect("Showing the meaning of life");
    });

    // Wait for our result to arrive.
    result_rx.await.ok()
}

fn calculate_meaning() -> MeaningOfLife {
    // Super long and complicated calculation.
    std::thread::sleep(Duration::from_millis(3000));
    
    MeaningOfLife(String::from("The meaning of life is 42."))
}

// ANCHOR_END: oneshot
// ANCHOR_END: all