// ANCHOR: all
fn main() {
    iced::run("Project Structure Example", App::update, App::view).unwrap();
}

// ANCHOR: app
#[derive(Debug, Clone)]
enum Message {
    // This message is used to handle the new views message
    NewMessage(new::Message),
    New,
}

#[derive(Debug, Default)]
enum View {
    #[default]
    Default,
    Edit(new::NewView),
}

#[derive(Debug, Default)]
struct App {
    view: View,
    items: Vec<String>,
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::NewMessage(view_message) => {
                if let View::Edit(edit) = &mut self.view {
                    // Call the update method of the edit view
                    // and handle the returned action
                    match edit.update(view_message) {
                        new::Action::None => {}
                        // If the action is a task, map the
                        // task to a message, the higher level message
                        new::Action::Task(task) => return task.map(Message::NewMessage),
                        new::Action::Submitted(content) => {
                            self.view = View::Default;
                            self.items.push(content);
                        }
                    }
                }
            }
            Message::New => {
                // Create a new view
                let (view, task) = new::NewView::new();
                self.view = View::Edit(view);
                // Run the task and map it to the higher level message
                return task.map(Message::NewMessage);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.view {
            View::Default => {
                let items = self
                    .items
                    .iter()
                    .map(|item| iced::widget::text(item).into())
                    .collect();
                iced::widget::column![
                    iced::widget::button("Edit").on_press(Message::New),
                    iced::widget::Column::from_vec(items)
                ]
                .into()
            }
            // If the view is an edit view, call the view method of the edit view
            // and map the returned message to the higher level message
            View::Edit(edit) => edit.view().map(Message::NewMessage),
        }
    }
}
// ANCHOR_END: app

// ANCHOR: new_view
mod new {
    // ANCHOR: action
    pub enum Action {
        None,
        Task(iced::Task<Message>),
        Submitted(String),
    }
    // ANCHOR_END: action

    #[derive(Debug, Clone)]
    pub enum Message {
        Submit,
        ChangeContent(String),
        RandomJoke,
    }

    #[derive(Debug, Default)]
    pub struct NewView {
        content: String,
    }

    impl NewView {
        pub fn new() -> (Self, iced::Task<Message>) {
            (Self::default(), Self::random_joke_task())
        }

        pub fn update(&mut self, message: Message) -> Action {
            match message {
                Message::Submit => Action::Submitted(self.content.clone()),
                Message::ChangeContent(content) => {
                    self.content = content;
                    Action::None
                }
                Message::RandomJoke => Action::Task(Self::random_joke_task()),
            }
        }

        pub fn view(&self) -> iced::Element<Message> {
            iced::widget::column![
                iced::widget::text_input("Content", &self.content).on_input(Message::ChangeContent),
                iced::widget::button("Random Joke").on_press(Message::RandomJoke),
                iced::widget::button("Submit").on_press(Message::Submit)
            ]
            .into()
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
}
// ANCHOR_END: new_view
// ANCHOR_END: all
