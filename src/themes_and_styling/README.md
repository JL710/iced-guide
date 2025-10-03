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

The built-in [`iced::Theme`](https://docs.rs/iced/latest/iced/enum.Theme.html) type has a [`Custom`](https://docs.rs/iced/latest/iced/enum.Theme.html#variant.Custom) variant which can be created using [`Theme::custom`](https://docs.rs/iced/0.13.1/iced/enum.Theme.html#method.custom) (or [`Theme::custom_with_fn`](https://docs.rs/iced/0.13.1/iced/enum.Theme.html#method.custom_with_fn) for greater control over the generated color palette).

If you need even more customization, you can create your own `Theme` type. The requirements are:
- Implement `Default` and [`DefaultStyle`](https://docs.rs/iced/0.13.1/iced/application/trait.DefaultStyle.html) (`iced::theme::Base` in later iced versions) for your custom type.
- For each widget you plan to support, implement its `Catalog` trait (if it has one) and the dependencies of that trait.

For a reference custom theme, see [`iced_material`](https://sr.ht/~pml68/iced_material). (This is written for iced 0.14, but the `Catalog` traits haven't changed much compared to 0.13)

## Styling

Per-widget styling is done via styling methods. These take `&Theme` (usually) and return a widget `Style`. The following examples are made with the button widget in mind, but most things apply to other widgets as well.

Most widgets include a default styling method, but some also have extras. You can use these by passing them to the widget's `style` method:

```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/styling.rs:builtin_style}}
```

You can also easily create static (or even dynamic) inline styles:

```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/styling.rs:inline_style}}
```

Notice the two underscores? They're for the `&Theme` and [`Status`](https://docs.rs/iced/latest/iced/widget/button/enum.Status.html) that get passed to our closure. What's this [`Status`](https://docs.rs/iced/latest/iced/widget/button/enum.Status.html), you ask? Well, a button may be hovered at a given moment, or it could be disabled, be pressed down, or neither. Taking this into account, lets see how we can create a dynamic styling method:

```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/styling.rs:style_method}}
```

```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/styling.rs:button_background}}
```

### Complete example

`main.rs`
```rust
{{#rustdoc_include {{code}}/themes-and-styling/src/styling.rs:all}}
```
