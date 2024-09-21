// ANCHOR: all
use iced::Task;

#[derive(Debug, Clone)]
enum Message {
    Refetch,
    CurrentIp(String),
}

struct App {
    ip: String,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App { ip: String::new() }, Task::none())
    }

    fn view(&self) -> iced::Element<Message> {
        iced::widget::column![
            iced::widget::text(&self.ip),
            iced::widget::button("Start task").on_press(Message::Refetch)
        ]
        .into()
    }

    // ANCHOR: update_function
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        println!("update");
        match message {
            // ANCHOR: return_custom_task
            Message::Refetch => return Task::perform(fetch_ip(), Message::CurrentIp),
            // ANCHOR_END: return_custom_task
            Message::CurrentIp(text) => {
                self.ip = text;
            }
        }
        Task::none()
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

fn main() {
    iced::run("Custom Task Example", App::update, App::view).unwrap();
}
// ANCHOR_END: all
