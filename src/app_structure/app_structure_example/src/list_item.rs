// ANCHOR: all

// ANCHOR: list_item_struct
// Depending on your use case, you can instead also
// accept types like `&str` or other references to your app state.
pub struct ListItem<'a, Message> {
    item: iced::Element<'a, Message>,
    on_delete: Option<Message>,
    on_edit: Option<Message>,
}
// ANCHOR_END: list_item_struct

// ANCHOR: builder
impl<'a, Message> ListItem<'a, Message> {
    // if you can, prefer using `impl Into` for other elements.
    // It makes the callsite look much nicer.
    pub fn new(item: impl Into<iced::Element<'a, Message>>) -> Self {
        Self {
            item: item.into(),
            on_delete: None,
            on_edit: None,
        }
    }

    pub fn on_delete(mut self, message: Message) -> Self {
        self.on_delete = Some(message);
        self
    }

    pub fn on_edit(mut self, message: Message) -> Self {
        self.on_edit = Some(message);
        self
    }
}
// ANCHOR_END: builder

// ANCHOR: from
impl<'a, Message> From<ListItem<'a, Message>> for iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    // Here you can put the code which builds the actual view.
    fn from(item_row: ListItem<'a, Message>) -> Self {
        let mut row = iced::widget::row![item_row.item]
            // In your viewable, you can handle things like spacing and alignment,
            // just like you would in your view function.
            .spacing(10);

        if let Some(on_delete) = item_row.on_delete {
            row = row.push(iced::widget::button("Delete").on_press(on_delete));
        }

        if let Some(on_edit) = item_row.on_edit {
            row = row.push(iced::widget::button("Edit").on_press(on_edit));
        }

        row.into()
    }
}
// ANCHOR_END: from
// ANCHOR_END: all
