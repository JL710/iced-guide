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
Use [`rfd::AsyncFileDialog`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html)'s [`pick_file`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html#method.pick_file) in combination with iced's [`Task`](https://docs.rs/iced/0.13.1/iced/struct.Task.html).

```rust
{{#rustdoc_include {{code}}/image-load-dialog/src/main.rs:open_image}}

```

When you pick a file/folder, it will return an `Option<`[`Filehandle`](https://docs.rs/rfd/latest/rfd/struct.FileHandle.html)`>`, as the user might change their mind.

We leverege the `Task`'s monadic api to make this implementation succinct and clean. 

### Complete Example

`main.rs`

```rust
{{#rustdoc_include {{code}}/image-load-dialog/src/main.rs:all}}
```

## Saving Files

To open a save-file dialog, use [`rfd::AsyncFileDialog`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html)'s [`save_file`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html#method.save_file)


```rust
{{#rustdoc_include {{code}}/save-dialog/src/main.rs:save_dialog}}
```

Similar to [`pick_file`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html#method.pick_file),  [`save_file`](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html#method.save_file) will return an an `Option<`[`Filehandle`](https://docs.rs/rfd/latest/rfd/struct.FileHandle.html)`>`, 

### Complete Example

`main.rs`

```rust
{{#rustdoc_include {{code}}/save-dialog/src/main.rs:all}}
```
