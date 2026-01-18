// ANCHOR: all
use iced::Task;

#[derive(Debug, Clone)]
enum Message {
    Refetch,
    CurrentIp(String),
}

#[derive(Default)]
struct App {
    ip: String,
}

impl App {
    fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::column![
            self.ip.as_str(),
            iced::widget::button("Start task").on_press(Message::Refetch)
        ]
        .into()
    }

    // ANCHOR: update_function
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        println!("update");

        match message {
            // ANCHOR: return_custom_task
            Message::Refetch => Task::perform(fetch_ip(), Message::CurrentIp),
            // ANCHOR_END: return_custom_task
            Message::CurrentIp(text) => {
                self.ip = text;
                Task::none()
            }
        }
    }
    // ANCHOR_END: update_function
}

// ANCHOR: fetch_ip
async fn fetch_ip() -> String {
    println!("fetch_ip");
    reqwest::get("https://api.seeip.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
// ANCHOR_END: fetch_ip

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
// ANCHOR_END: all
