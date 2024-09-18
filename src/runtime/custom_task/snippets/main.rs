// ANCHOR: all
use iced::{Application, Command};

#[derive(Debug, Clone)]
enum Message {
    Refetch,
    CurrentIp(String),
}

struct App {
    ip: String,
}

impl Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();
    type Executor = iced::executor::Default;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App { ip: String::new() }, Command::none())
    }

    fn title(&self) -> String {
        String::new()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        iced::widget::column![
            iced::widget::text(&self.ip),
            iced::widget::button("Start Command").on_press(Message::Refetch)
        ]
        .into()
    }

    // ANCHOR: update_function
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        println!("update");
        match message {
            // ANCHOR: return_custom_command
            Message::Refetch => return Command::perform(fetch_ip(), Message::CurrentIp),
            // ANCHOR_END: return_custom_command
            Message::CurrentIp(text) => {
                self.ip = text;
            }
        }
        Command::none()
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
    App::run(iced::Settings::default()).expect("Application raised an error");
}
// ANCHOR_END: all
