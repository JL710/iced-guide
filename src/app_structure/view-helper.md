
# The View-Helper Pattern

The view-helper pattern is so simple, it's hard to even call it a pattern.

You can use a view-helper to make your code more readable or to make a part of your view reusable.

```rust
pub fn list(items: &[String], on_delete: impl Fn(usize) -> Message) -> iced::Element<'_, Message> {
    iced::widget::column(
        items.iter()
            .enumerate()
            .map(|(index, item)| {
                iced::row![
                    iced::widget::text(item),
                    iced::widget::button(iced::widget::text("Delete"))
                        .style(iced::widget::button::danger)
                        .on_press(on_delete(index)),
                ].into()
            })
    ).into()
}
```

Now you can just call this function inside your main view:

```rust
impl App {
    pub fn view(&self) -> Element<'_, Message> {
        list(self.items.as_slice(), |index| Message::Delete(index))
    }
}
```

## Conclusion

This approach is great because of it's simplicity and versatility.
You can create these view-helpers, just as you would any other helper function.

A disadvantage of this approach, is that more complex views may require
a lot more function parameters or an additional helper struct.

You can also check out the [Viewable Pattern](./viewable.md) as a more sophisticated alternative.
