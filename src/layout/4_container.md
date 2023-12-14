# Container

The Container is useful when aligning items. A Container has one child element (could be a button, text, column, row, etc.).

```rust,ignore
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
> We use `width` and `height` to maximize the size of the Container.
> This makes space for centering. 
>
> This it not exactly necessary as long as the container is large enough.
> If there is no space you will not see a difference between applied alignment and none.

Both [`align_x`](https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.align_x) and [`align_y`](https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.align_y) methods are available for alignment purposes on [`Container`](https://docs.rs/iced/latest/iced/widget/struct.Container.html).
