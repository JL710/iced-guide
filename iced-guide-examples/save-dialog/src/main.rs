// ANCHOR: all
use std::path::PathBuf;

use iced::widget::{button, center};
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application("Save File Dialog Example", App::update, App::view).run()
}

#[derive(Debug, Clone)]
pub enum Message {
    Save,
    Exported(Result<PathBuf, Error>),
    ExportCancelled,
}

#[derive(Debug, Default)]
struct App;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Save => return export_build_info(),
            Message::Exported(Ok(destination)) => {
                let _ = open::that(destination);
            }
            Message::Exported(Err(_error)) => (),
            Message::ExportCancelled => (),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        center(button("Export Build Information").on_press(Message::Save)).into()
    }
}

// ANCHOR: save_dialog
pub fn export_build_info() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .set_file_name("build info.txt")
            .save_file(),
    )
    .then(|handle| match handle {
        Some(handle) => Task::perform(save_build_info(handle), Message::Exported),
        None => Task::done(Message::ExportCancelled),
    })
}
// ANCHOR_END: save_dialog

#[derive(Debug, Clone)]
pub enum Error {
    Io { reason: String },
}

pub async fn save_build_info(handle: rfd::FileHandle) -> Result<PathBuf, Error> {
    static BUILD_INFO: &str = "Build Date: 07/07/2025\nVersion: 1.2.3";

    handle
        .write(BUILD_INFO.as_bytes())
        .await
        .map(|_| handle.path().to_path_buf())
        .map_err(|error| Error::Io {
            reason: error.to_string(),
        })
}
// ANCHOR_END: all
