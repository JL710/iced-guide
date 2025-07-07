// ANCHOR: all
#[derive(Debug, Clone)]
enum Message {
    CalculatedInformation(i32),
    StartCalculatingInformation,
}

#[derive(Default)]
struct App {
    hard_to_process_information: Option<i32>,
    calculation_in_progress: bool,
}

impl App {
    // ANCHOR: update
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::CalculatedInformation(information) => {
                // Set the information
                self.hard_to_process_information = Some(information);
                self.calculation_in_progress = false;
            }
            Message::StartCalculatingInformation => {
                // Change the state to indicate that the calculation is in progress
                self.calculation_in_progress = true;
                // Return a task that will calculate the information
                // ANCHOR: compute_task
                return iced::Task::future(async {
                    let information = tokio::task::spawn_blocking(|| {
                        println!("Calculating information...");
                        // Simulate a long computation
                        std::thread::sleep(std::time::Duration::from_secs(2));

                        // return some value
                        42
                    })
                    .await
                    .unwrap();

                    // Send the information back to the update function
                    Message::CalculatedInformation(information)
                });
                // ANCHOR_END: compute_task
            }
        }
        iced::Task::none()
        // ANCHOR_END: update
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::column![
            // Display the information if it is available
            iced::widget::Text::new(self.hard_to_process_information.map_or(
                "Information will be ready in a second...".to_string(),
                |x| format!("Information: {}", x),
            )),
            // Display a button to start the calculation
            iced::widget::button("Start Calculation").on_press_maybe(
                if self.calculation_in_progress {
                    None
                } else {
                    Some(Message::StartCalculatingInformation)
                }
            )
        ]
        .into()
    }
}

fn main() {
    iced::run("Task Example", App::update, App::view).unwrap();
}
// ANCHOR_END: all
