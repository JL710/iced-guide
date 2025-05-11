
# The Viewable Pattern

The viewable pattern is an extension of the view-helper pattern,
which allows for a bit cleaner code on the call site.

A viewable is a Struct which is build during the view function in your app and implements `Into<iced::Element>`.

In practive, it behaves and is used like any other iced widget,
it may contain other [`iced::Element`](https://docs.rs/iced/latest/iced/type.Element.html)s or references to your app state, like a `&str`.

## Dependencies

To create a viewable, we'll start with creating it's struct, which contains all dependencies we'll need to build our view tree later.

A viewable should avoid owning data and instead prefer references, if possible.

```rust
{{#rustdoc_include app_structure_example/src/list_item.rs:list_item_struct}}
```

In our case, we want to display an arbitrary [`Element`](https://docs.rs/iced/latest/iced/type.Element.html), which could be a text, an image or maybe even a row with both.

Additionally, we want enable adding a delete and an edit button.
Since these buttons will always look the same,
all we need is the Message which should be triggered when the button is clicked.

> **NOTE:** If you are familiar enough with rust's type system, you could also use

To make those additional buttons optional, we'll use an `Option<Message>`.

## The Builder

A viewable usually uses the builder pattern to simplify creation and make the callsite more readable.
The recommended builder pattern requests all mandatory dependencies in the constructor, while adding optional ones with chainable methods.

> **NOTE:** If you are familiar enough with Rust's type system, you could also use a typestate builder to ensure all required dependencies are provided
>  and to control the addition of optional elements.
>
> In practice, this isn't always the best idea, because it creates more boilerplate and duplicate code.

```rust
{{#rustdoc_include app_structure_example/src/list_item.rs:builder}}
```


## Creating The View

The last part of creating a Viewable is to build the actual view tree, as you would for your application.

To allow calling `.into()` as we would with a normal widget, we'll implement the From trait for [`iced::Element`](https://docs.rs/iced/latest/iced/type.Element.html).

```rust
{{#rustdoc_include app_structure_example/src/list_item.rs:from}}
```

If you want to support custom themes or additional renderers,
you'll have to specify the additional generic parameters for [`iced::Element`](https://docs.rs/iced/latest/iced/type.Element.html)
and set the constraints according to what you need.

## Using The Viewable

to use the viewable, we can leverage the builder pattern we just implemented, followed by a call to `.into()`.

In this case, we're only using `on_delete`.
This will cause the viewable to add a delete button, but to forfeit the edit button.

```rust
{{#rustdoc_include app_structure_example/src/main.rs:viewable}}
```

## Conclusion

The viewable pattern is great way to build your own "widgets",
especially since you can make using them really ergonomic.

In some cases, a viewable might be overkill - you may be interested in the [View-Helper](./view-helper.md) for those times.

A Viewable also can't hold any application state.
For that, you could take a look at the [Component Pattern](./component.md).
