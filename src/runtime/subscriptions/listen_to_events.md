# Listen to events

Often you want to do something when the user presses some key, if a file is dropped on your window or general mouse events. 
For that, you can use the [`iced::event::listen`](https://docs.iced.rs/iced/event/fn.listen.html) subscription. It runs in the background and emits a message with/on every [`Event`](https://docs.iced.rs/iced/enum.Event.html).

> **Note:** If you just want to get mouse events in a specific widget area you should use the [`MouseArea`](https://docs.iced.rs/iced/widget/struct.MouseArea.html) widget. 

Here is a practical example how to listen to an arbitrary event in form of a keyboard event.

In the example, the subscription always runs as defined in the `.subscription` method of application here:
```rust
{{#rustdoc_include {{code}}/subscription-listen-to-events/src/main.rs:main}}
```

It emits a message containing the event:
```rust
{{#rustdoc_include {{code}}/subscription-listen-to-events/src/main.rs:message_enum}}
```

In the update method we can use that event and react to it:
```rust
{{#rustdoc_include {{code}}/subscription-listen-to-events/src/main.rs:update}}
```

> **Important Note**: The example uses [`iced::event::listen`](https://docs.rs/iced/0.13.1/iced/event/fn.listen.html) that reacts to all kind of events. There are specific subscriptions for special event kinds, such as [window](https://docs.iced.rs/iced/window/fn.events.html), [key_press](https://docs.iced.rs/iced/keyboard/fn.on_key_press.html) and [key_release](https://docs.iced.rs/iced/keyboard/fn.on_key_release.html), as well. 

## Full Code
```rust
{{#rustdoc_include {{code}}/subscription-listen-to-events/src/main.rs:all}}
```
