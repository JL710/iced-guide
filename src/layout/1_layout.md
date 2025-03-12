# Layout

In this section, you will learn about different layouting widgets and techniques that iced offers us to align and place widgets. In addition, you will learn some of the basic layouting practices. These layout techniques are used to maintain best UI structure and maintain responsiveness.

## Element.explain
When styling a GUI, often the result does not look exactly what you wanted it to look like. Therefore, you debug your layout.

In web development, you can use the inspection tool of your browser to show the layout and borders of your elements.

In iced, we don't have an inspection tool. But we have the [`Element.explain`](https://docs.rs/iced/latest/iced/type.Element.html#method.explain) function that we can apply to any [`Element`](https://docs.rs/iced/latest/iced/type.Element.html).
This function will draw a line around the element and all of its children. With that, you can debug how spacing and sizing are applied by the renderer.

Here is a short snipped that uses `.explain`:
```rust
iced::Element::new(your_widget).explain(iced::Color::BLACK)
```
