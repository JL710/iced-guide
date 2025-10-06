
# The Viewable Pattern

The viewable pattern is an extension of the view-helper pattern,
which allows for a bit cleaner code on the call site.

A viewable is a Struct which is build during the view function in your app and implements `Into<iced::Element>`.

In practice, it behaves and is used like any other iced widget,
it may contain other [`iced::Element`](https://docs.rs/iced/0.13.1/iced/type.Element.html)s or references to your app state, like a `&str`.

## What we want to achieve

Let's imagine we want to have a list with some additional options.

In our case, we want to display an arbitrary [`Element`](https://docs.rs/iced/0.13.1/iced/type.Element.html),
which could be a text, an image or maybe even a row with both.

Additionally, we want enable adding a delete and an edit button.
Since these buttons will always look the same,
all we need is the `Message` which should be triggered when the button is clicked.

To make those additional buttons optional, we'll use an `Option<Message>`.

If we were to use the view-helper, it could look like this:

```rust
fn list_item<'a, Message>(item: iced::Element<'a, Message>, on_delete: Option<Message>, on_edit: Option<Message>) -> iced::Element<'a, Message> {
    // We want the buttons next to the item itself, so we'll use a row.
    let mut row = iced::widget::row![item]
        .spacing(10);

    // Not all lists might allow deleting or editing, so use Options for the on_delete and on_edit Messages.
    // We only add the button if the message is already set.
    if let Some(on_delete) = on_delete {
        row = row.push(iced::widget::button("Delete").on_press(on_delete));
    }

    if let Some(on_edit) = on_edit {
        row = row.push(iced::widget::button("Edit").on_press(on_edit));
    }

    row.into()
}
```

Now if we want to call this helper, the call site would look like this.

```rust
list_item(text("I'm item 1"), Message::Delete(1), Message::Edit(1))
```

This works, but if we wanted to add additional buttons (e.g. duplicate) we'd have to add more and more parameters.
After a while our view-helper starts to get less readable.

Wouldn't it be nice if we could instead call it, like it was a widget?

```rust
list_item(text("I'm item 1")
    .on_delete(Message::Delete(1))
    .on_edit(Message::Edit(1))
    .into()
```

That's what the Viewable pattern is all about.
It allows you to specify options one by one to create a view with.

In the next steps, you'll learn how to transform our view-helper `list_item` into a viewable.

## Dependencies

To create a viewable, we'll start with creating it's struct, which contains all dependencies we'll need to build our view tree later.

For this, we just pack all the parameters into a struct like this

```rust
{{#rustdoc_include {{code}}/app-structure/src/list_item.rs:list_item_struct}}
```

Then we just change the function to accept a `ListItem` struct.

```rust
fn list_item<'a, Message>(list_item: ListItem<'a, Message>) -> iced::Element<'a, Message> {
    // We want the buttons next to the item itself, so we'll use a row.
    let mut row = iced::widget::row![list_item.item]
        .spacing(10);

    // Not all lists might allow deleting or editing, so use Options for the on_delete and on_edit Messages.
    // We only add the button if the message is already set.
    if let Some(on_delete) = list_item.on_delete {
        row = row.push(iced::widget::button("Delete").on_press(on_delete));
    }

    if let Some(on_edit) = list_item.on_edit {
        row = row.push(iced::widget::button("Edit").on_press(on_edit));
    }

    row.into()
}
```

That doesn't change all that much though. With this change our callsite would look like this:

```rust
list_item(ListItem {
        item: "I'm item 1",
        on_delete: Some(Message::Delete(1)),
        on_edit: Some(Message::Edit(1)),
    }
);
```

We can already gain a small improvement like this:

```rust
impl<'a, Message> ListItem<'a, Message> {
    pub fn into_element(self) -> Element<'a, Message> {
        let mut row = iced::widget::row![item_row.item]
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
```

Using a method, we can create our `ListItem` like this:

```rust
ListItem {
    item: "I'm item 1",
    on_delete: Some(Message::Delete(1)),
    on_edit: Some(Message::Edit(1)),
}.into_element()
```

That's better, but the official widgets use `.into()`, not a custom method.
For that, we'll have to implement the `From` trait.

In most cases you can just copy `impl` block and function signature like shown here:

```rust
{{#rustdoc_include {{code}}/app-structure/src/list_item.rs:from}}
```

If you want to support custom themes or additional renderers,
you'll have to specify the additional generic parameters for [`iced::Element`](https://docs.rs/iced/0.13.1/iced/type.Element.html)
and set the constraints according to what you need. Since we're using the default theme and renderer, we can skip this step.

## The Builder

Specifying options by chaining commands, like shown above, is called the builder pattern.
It's commonly used in object oriented languages, but is especially common in Rust.

A viewable usually uses the builder pattern to simplify creation and make the callsite more readable.
The recommended builder pattern requests all mandatory dependencies in the constructor, while adding optional ones with chainable methods.

> **NOTE:** If you are familiar enough with Rust's type system, you could also use a typestate-builder to ensure all required dependencies are provided
>  and to control the addition of optional elements.
>
> In practice, this isn't always the best idea, because it creates more boilerplate and duplicate code.

```rust
{{#rustdoc_include {{code}}/app-structure/src/list_item.rs:builder}}
```

## Using The Viewable

to use the viewable, we can leverage the builder pattern we just implemented, followed by a call to `.into()`.

In this case, we're only using `on_delete`.
This will cause the viewable to add a delete button, but to forfeit the edit button.

```rust
impl App {
    fn view(&self) -> iced::Element<Message> {
    {{#include {{code}}/app-structure/src/main.rs:viewable}}
    }
}
```

## Conclusion

The viewable pattern is great way to build your own pseudo-widgets,
especially since you can make using them really ergonomic.

In some cases, a viewable might be overkill - you may be interested in the [View-Helper](./view-helper.md) for those times.

A Viewable also can't hold any application state.
For that, you could take a look at the [Component Pattern](./component.md).
