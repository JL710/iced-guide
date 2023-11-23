# Columns and Rows

The two most important structs for laying out widgets are [`Column`](https://docs.rs/iced/latest/iced/widget/struct.Column.html) and [`Row`](https://docs.rs/iced/latest/iced/widget/struct.Row.html).

Both lay out their children in one direction. The column organizes the widgets vertically and the row horizontally.

![Column and Row Example](./column_row.drawio.svg)

By default, they align the items in the top left corner of their space.

A convenient way to create columns and rows is with the [`column!`](https://docs.rs/iced/latest/iced/widget/macro.column.html) and [`row!`](https://docs.rs/iced/latest/iced/widget/macro.row.html) macros.

We saw one of them in the [Minimal Application - Counter](../../minimal-example/index.md).
```rust,ignore
{{#rustdoc_include ../../minimal-example/main.rs:column}}
```
There, we created a [`Column`](https://docs.rs/iced/latest/iced/widget/struct.Column.html) with three children inside. One text and two buttons. The syntax for rows is the same.

You can put any [`Element`](https://docs.rs/iced_core/0.10.0/iced_core/struct.Element.html) inside a [`Column`](https://docs.rs/iced/latest/iced/widget/struct.Column.html) or [`Row`](https://docs.rs/iced/latest/iced/widget/struct.Row.html).


## Alignment
Of course, we can change the horizontal alignment for columns and the vertical alignment for rows.

![Column and Row Example](./column_row.drawio.svg)
This is how they would align in the center.

In code, if you want to set the [`Alignment`](https://docs.rs/iced/latest/iced/enum.Alignment.html) you can call the [`align_items`](https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.align_items) method on your column/row. It will return itself with the new alignment.
```rust, ignore
let some_column = iced::widget::column![
    iced::widget::text("Hello World!"),
    iced::widget::text("Another Hello World!")
].align_items(iced::Alignment::Center)
```


## Spacing
Since you can not set a margin in iced and often want to have a spacing between elements.

Columns and rows have the [`spacing`](https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.spacing) method to set the spacing.

Here is an example of how to use spacing on a column:
```rust, ignore
let some_column = iced::widget::column![
    iced::widget::text("Hello World!"),
    iced::widget::text("Another Hello World!")
].spacing(20)
```

![Spacing Image](./column_spacing.drawio.svg)