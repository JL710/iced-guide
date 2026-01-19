// ANCHOR: all
use iced::Alignment::Center;
use iced::widget::{button, center, column, image};
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Error {
    InvalidImage,
}

#[derive(Debug, Clone)]
enum Message {
    OpenImage,
    ImageLoaded(Result<image::Handle, Error>),
    FileCancelled,
}

#[derive(Default)]
struct App {
    loaded_image: Option<image::Handle>,
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        center(
            column![button("Open Image").on_press(Message::OpenImage)]
                .push(self.loaded_image.as_ref().map(image))
                .align_x(Center),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenImage => open_image(),
            Message::ImageLoaded(result) => match result {
                Ok(handle) => {
                    self.loaded_image = Some(handle);
                    Task::none()
                }
                Err(_) => Task::none(),
            },
            Message::FileCancelled => Task::none(),
        }
    }
}

// ANCHOR: open_image
fn open_image() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .add_filter( // <-- (OPTIONAL) Add a filter to only allow PNG and JPEG formats.
                "Image Formats",
                &["png", "jpg", "jpeg"],
            )
            .pick_file(), // <-- Launch the dialog window.
    )
    .then(|handle| match handle {
        // After obtaining a file handle from the dialog, we load the image.
        //
        // We use Task::perform to run load_image, as this may take a while to load.
        Some(file_handle) => Task::perform(load_image(file_handle), Message::ImageLoaded),

        // The user has cancelled the operation, so we return a "Cancelled" message.
        None => Task::done(Message::FileCancelled),
    })
}

/// Simplified code to load an image.
///
/// In practice, you may explore other options,
/// but this goes beyond the scope of this tutorial.
async fn load_image(handle: rfd::FileHandle) -> Result<image::Handle, Error> {
    Ok(image::Handle::from_path(handle.path()))
}
// ANCHOR_END: open_image

// ANCHOR_END: all
