# Widget API

> **Disclaimer:** This section is based on version 0.13.1! If you are using a newer version of Iced the API will be different!

In rare cases, you might want to write a custom widget. A custom widget gives you full control over the layout and drawing process. A custom widget is defined as something that implements the [`Widget`](https://docs.rs/iced/0.13.1/iced/advanced/trait.Widget.html) trait.

You might want to use other existing widgets in your custom widget. It is recommended that you store them as `Element`s in your widget struct. When calling methods on the children that take the state `Tree`, you need to make sure to pass the correct child tree from `Tree.children` to the child widget.  

To get detailed examples of the `Widget` trait you can always take a look at the core widgets like `Button`.

## Generics of the Widget trait

The `Widget` trait has three different generics.

### Message

The `Message` generic is the message type that your widget can emit. It should be left generic to fit into any application that wants to use the widget. That is why it is common not to create the message hardcoded, but to take a Message as a parameter or a function that generates the appropriate message instead.

Good examples of this are `Button` where the message can be set with the [`on_press`](https://docs.rs/iced/0.13.1/iced/widget/struct.Button.html#method.on_press) and is saved in the `Button` struct:
```rust
pub fn on_press(mut self, on_press: Message) -> Self {
    self.on_press = Some(OnPress::Direct(on_press));
    self
}
```
or the `TextInput` that takes a message in its [`on_input`](https://docs.rs/iced/0.13.1/iced/widget/text_input/struct.TextInput.html#method.on_input) method to generate the appropriate message:
```rust
pub fn on_input(
    mut self,
    on_input: impl Fn(String) -> Message + 'a,
) -> Self {
    self.on_input = Some(Box::new(on_input));
    self
}
```

### Theme

The `Theme` generic specifies the type of the theme. To keep it simple, you could stay with the iced provided ones and use `iced::Theme`.

In the long run, it is better to use a generic theme and Catalog instead.

### Renderer

The `Renderer` generic specifies the renderer that is used to draw your widget. 
For most widgets, you can use a generic that implements `iced::advanced::Renderer`, since this trait has the renderer interface that is most used (as far as I can tell). 
There are also traits like the [one for SVGs](https://docs.rs/iced/0.13.1/iced/advanced/svg/trait.Renderer.html) that you might need for specific use cases, such as SVGs.

## Displaying Text

Displaying text in your widget is quite difficult. The easiest way I found is to just use a normal text widget instead of using the text renderer directly.
