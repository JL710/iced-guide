# Svg

This is an example of including a svg in the application binary and using it in a performant/optimized way, where the handle is only created from the `.svg` once (using `LazyLock`).

```rust
use iced::widget::svg::{Handle, Svg};
use std::sync::LazyLock;

// the handle is only created once
pub static ARROW_LEFT: LazyLock<Handle> =
    LazyLock::new(|| Handle::from_memory(include_bytes!("arrow-left.svg")));

// this can be called in view to get a ready to use svg
pub fn arrow_left() -> Svg<'_> {
    Svg::new(ARROW_LEFT.clone()).width(iced::Shrink)
}
```
