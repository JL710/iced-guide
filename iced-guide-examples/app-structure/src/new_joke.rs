// ANCHOR: all

// ANCHOR: message
#[derive(Debug, Clone)]
pub enum Message {
    ChangeContent(String),
    RandomJoke,
    Submit,
    Cancel,
}
//ANCHOR_END: message

// ANCHOR: action
pub enum Action {
    // The user was happy with the joke and wants to submit it to the list
    Submit(String),
    // The user wants to cancel adding a new joke
    Cancel,
    // The compositions needs to run a task
    Run(iced::Task<Message>),
    // The composition does not require any additional actions
    None,
}
// ANCHOR_END: action

// ANCHOR: state
pub struct NewJoke {
    joke: String,
}
// ANCHOR_END: state

// ANCHOR: new
impl NewJoke {
    pub fn new() -> Self {
        Self {
            joke: String::new(),
        }
    }
}
// ANCHOR_END: new

// ANCHOR: view
impl NewJoke {
    pub fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::column![
            iced::widget::text_input("Content", &self.joke)
                // on_input expects a closure, which would usually look like this:
                // |new_value| Message::ChangeContent(new_value)
                // Thankfully, you can just use the enum variants name directly
                .on_input(Message::ChangeContent),
            iced::widget::button("Random Joke").on_press(Message::RandomJoke),
            iced::widget::row![
                iced::widget::button("Cancel").on_press(Message::Cancel),
                iced::widget::button("Submit").on_press(Message::Submit)
            ]
            .spacing(10),
        ]
        .padding(10)
        .spacing(10)
        .into()
    }
}
// ANCHOR_END: view

// ANCHOR: update
impl NewJoke {
    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => Action::Submit(self.joke.clone()),
            Message::Cancel => Action::Cancel,
            Message::ChangeContent(content) => {
                self.joke = content;
                Action::None
            }
            Message::RandomJoke => Action::Run(Self::random_joke_task()),
        }
    }

    fn random_joke_task() -> iced::Task<Message> {
        iced::Task::future(async {
            // Fetch a joke from the internet
            let client = reqwest::Client::new();
            let response: serde_json::Value = client
                .get("https://icanhazdadjoke.com")
                .header("Accept", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            // Parse the response
            let joke = response["joke"].as_str().unwrap();

            // Return the joke as a message
            Message::ChangeContent(joke.to_owned())
        })
    }
}
// ANCHOR_END: update

// ANCHOR_END: all
