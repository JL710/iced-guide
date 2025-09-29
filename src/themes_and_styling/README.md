# Themes and styling

Iced lets you customize the look and feel of your app on multiple levels:
- Singular widget instances can have unique looks.
- Themes can define their own colorschemes and base widget styles, which are used by default.

## Themes

You can set the theme used by your app using `Application::theme`:

```rust
fn main() -> iced::Result {
    iced::application(...)
        .theme(|_| iced::Theme::Dracula)
        .run()
}
```

A nicer way to do this is by storing the theme in your app struct, then implementing a `theme` method (or something similar) for it which looks something like this:

```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/themes.rs:theme_fn}}
```

Then you can use it like so:

```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/themes.rs:main}}
```

### Complete example

`main.rs`
```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/themes.rs:all}}
```

## Custom themes

The built-in `iced::Theme` type has a `Custom` variant which can be created using [`Theme::custom`](https://docs.rs/iced/0.13.1/iced/enum.Theme.html#method.custom) (or [`Theme::custom_with_fn`](https://docs.rs/iced/0.13.1/iced/enum.Theme.html#method.custom_with_fn) for greater control over the generated color palette).

If you need even more customization, you can create your own `Theme` type. The requirements are:
- Implement `Default` and [`DefaultStyle`](https://docs.rs/iced/0.13.1/iced/application/trait.DefaultStyle.html) (`iced::theme::Base` in later iced versions) for your custom type.
- For each widget you plan to support, implement its `Catalog` trait (if it has one) and the dependencies of that trait.

For a reference custom theme, see [`iced_material`](https://sr.ht/~pml68/iced_material). (This is written for iced 0.14, but the `Catalog` traits haven't changed much compared to 0.13)
