# Container

The Container is useful when aligning items. A Container has one child element (could be a button, text, column, row, etc.).

```rust
use iced::{widget, Length};
use iced::alignment::{Horizontal, Vertical};

let stuff_centered = widget::Container::new(widget::text("Some Text"))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .height(Length::Fill);
```
> **Note:**
>
> We use `width` and `height` to maximize the size of the container, creating extra space for centering.
>
> However, this is not strictly necessary if the container is already large enough;
> without additional space, there will be no noticeable difference between applying alignment and not applying any alignment.

Both [`align_x`](https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.align_x) and [`align_y`](https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.align_y) methods are available for alignment purposes on [`Container`](https://docs.rs/iced/latest/iced/widget/struct.Container.html).
