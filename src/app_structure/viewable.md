
## The Viewable Pattern

A viewable is a Struct which is build during the view function in your app and implements `into<iced::Element>`.

In practive, it behaves and is used like any other iced widget,
it may contain other `iced::Element`s or references to your app state, like a `&str`.

```rust
{{#rustdoc_include app_structure_example/src/main.rs:viewable}}
```
