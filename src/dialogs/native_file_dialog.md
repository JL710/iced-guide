# OS-Native Dialogs
To create native-looking dialogs, you can use the [`Rusty File Dialog`](https://crates.io/crates/rfd) crate.

## Opening Files
### Scenario
Imagine that you're creating a simple image viewer, and you want to load an image you've recently downloaded.

You have a simple prototype, but not sure what to do at `open_image`:

```rust
use iced::widget::image;

enum Error {
    InvalidImage,
}

pub enum Message {
    /// Open the file dialog.
    OpenImage,

    /// An image has been loaded.
    ImageLoaded(Result<image::Handle, Error>),

    /// The user changed their mind.
    FileCancelled
}

// Your app's update function.
pub fn update(&mut self, message: Message) -> Task<Message> {
    match self {
        Message::OpenImage => open_image(),

        /* Additional code omitted for simplicity */
        _ => Task::none()
    }
}

pub fn open_image() -> Task<Message> {
    todo!()
}

```

### Solution
Use [`rfd::AsyncFileDialog`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html)'s [`pick_file`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html#method.pick_file) in combination with iced's [`Task`](https://docs.rs/iced/latest/iced/struct.Task.html).

```rust
fn open_image() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .add_filter( // <-- (OPTIONAL) Add a filter to only allow PNG and JPEG formats.
                "Image Formats",
                &["png", "jpg", "jpeg"], 
            )
            .pick_file() // <-- Launch the dialog window.
    )
    .then(|handle| match handle {
        // After obtaining a file handle from the dialog, we load the image.
        //
        // We use Task::perform to run load_image, as this may take a while to load.
        Some(file_handle) => {
            Task::perform(
                load_image(file_handle), 
                Message::ImageLoaded
            )
        },

        // The user has cancelled the operation, so we return a "Cancelled" message.
        None => Task::done(Message::FileCancelled)
    })
}

/// Simplified code to load an image. 
/// 
/// In practice, you may explore other options,
/// but this goes beyond the scope of this tutorial.
async fn load_image(handle: rfd::FileHandle) -> Result<image::Handle, Error> {
    Ok(image::Handle::from_path(handle.path()))
}

```

When you pick a file/folder, it will return an `Option<`[`Filehandle`](https://docs.rs/rfd/latest/rfd/struct.FileHandle.html)`>`, as the user might change their mind.

We leverege the `Task`'s monadic api to make this implementation succinct and clean. 


### Complete Example

`Cargo.toml`:

```rust
[package]
name = "file_dialog"
version = "0.1.0"
edition = "2024"

[dependencies]
iced = { version = "0.13.1", features = ["image"] }
rfd = "0.15"
```

`main.rs`

```rust
use iced::Alignment::Center;
use iced::widget::{button, center, column, image};
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application("File Dialog Example", App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Error {
    InvalidImage
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
    pub fn view(&self) -> Element<Message> {
        center(
            column![button("Open Image").on_press(Message::OpenImage)]
                .push_maybe(self.loaded_image.as_ref().map(image))
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

fn open_image() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .add_filter(
                "Images Formats",
                &["png", "jpg", "jpeg"],
            )
            .pick_file(),
    )
    .then(|handle| match handle {
        Some(file_handle) => Task::perform(
            load_image(file_handle), 
            Message::ImageLoaded
        ),

        None => Task::done(Message::FileCancelled),
    })
}

async fn load_image(handle: rfd::FileHandle) -> Result<image::Handle, Error> {
    Ok(image::Handle::from_path(handle.path()))
}

```