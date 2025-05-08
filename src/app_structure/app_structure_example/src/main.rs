mod list_item;
mod new_joke;

// ANCHOR: all
fn main() {
    iced::run("Project Structure Example", App::update, App::view).unwrap();
}

// ANCHOR: app
#[derive(Debug, Clone)]
enum Message {
    // This message is used to handle the new views message
    NewJoke(new_joke::Message),
    OpenNewJokeComponent,
    Delete(usize),
}

#[derive(Default)]
// ANCHOR: view_enum
enum View {
    #[default]
    ListJokes,
    // This holds our new joke components state
    NewJoke(new_joke::NewJoke),
}
// ANCHOR_END: view_enum

// ANCHOR: app_state
#[derive(Default)]
struct App {
    view: View,
    items: Vec<String>,
}
// ANCHOR_END: app_state

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // ANCHOR: update_component
            Message::NewJoke(view_message) => {
                // as with all enums in rust, we'll need to use an if-let expression
                // to get access to our component from the `View` enum
                if let View::NewJoke(edit) = &mut self.view {
                    // Call the update method of the edit view
                    // and handle the returned action
                    match edit.update(view_message) {
                        // The none action is a no-op
                        new_joke::Action::None => {}

                        // If the action is a task, we'll need to map it, to ensure it returns the right Message type.
                        // This is the exact same as with `view` and the returned `iced::Element`
                        new_joke::Action::Run(task) => return task.map(Message::NewJoke),

                        // If the action is a cancel, switch back to the list view
                        new_joke::Action::Cancel => {
                            self.view = View::ListJokes;
                        }

                        // If the action is a submit, add the new joke before returning to the list view
                        new_joke::Action::Submit(new_joke_content) => {
                            self.view = View::ListJokes;
                            self.items.push(new_joke_content);
                        }
                    }
                }
            }
            // ANCHOR_END: update_component
            // ANCHOR: create_component
            Message::OpenNewJokeComponent => {
                // Create a new component
                let component = new_joke::NewJoke::new();
                self.view = View::NewJoke(component);
            }
            // ANCHOR_END: create_component
            Message::Delete(index) => {
                self.items.remove(index);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.view {
            View::ListJokes => {
                // ANCHOR: viewable
                let items = self
                    .items
                    .iter()
                    // since we want deletion, we'll need the index of each item, so we know which one to delete
                    .enumerate()
                    .map(|(index, item)| {
                        // create a listitem for each joke
                        list_item::ListItem::new(iced::widget::text(item))
                            // save the index to delete in the message
                            .on_delete(Message::Delete(index))
                            // since we implemented the `From` trait, we can just use into() to create an element,
                            // just as if we were using a widget
                            .into()
                    })
                    .collect();

                iced::widget::column![
                    iced::widget::button("New").on_press(Message::OpenNewJokeComponent),
                    iced::widget::Column::from_vec(items)
                ]
                // Some spacing goes a long way to make your UI more visually appealing
                .spacing(10)
                .into()
                // ANCHOR_END: viewable
            }
            // If the view is an edit view, call the view method of the edit view
            // and map the returned message to the higher level message
            // ANCHOR: new_joke_view
            View::NewJoke(new_joke) => new_joke.view().map(Message::NewJoke),
            // ANCHOR_END: new_joke_view
        }
    }
}
// ANCHOR_END: app

// ANCHOR_END: all
