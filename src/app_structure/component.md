# Structuring Apps
When you create larger iced apps, you might want to have reusable components and views.
For that, there is a common approach that is also used in this [showcase](https://github.com/hecrj/icebreaker) from the founder of iced.

In this architecture a view/component is independent of the application and is split into three parts, message, the state and the action.

The state will have a normal view function that returns a `iced::Element<Message>`.

The update function will differ a bit. Instead of a Task like our main application, it will return an action enum.
Common variants for this enum can be `None`, `Task(iced::Task<Message>)` and `Submit(String)`. You can think of this action like a message that is sent from the view to the parent application that should handle it.

This kind of architecture will enhance composability.

## Initial State
The view should also have a function that creates a new instance of that view. 
If some task needs to be done for the initial state of the view (i.e. fetching data), something like `(Self, iced::Task<Message>)` should be returned by that function.
The task would be executed, and the data is sent via a message to the view.

To differ between a loaded and not loaded state you can use something like this pattern for your state:
```rs
enum State {
    NotLoaded,
    Loaded {
        field1: String,
        field2: i32
    }
}
```
If you do not need to lazy load something from an API or do similar, you can just have a normal struct as state.

## Mapping
If you want to compose your UI of subparts that have their own functionality, it makes sense to give them their own messages.

But to use a `iced::Element<ViewMessage>` in your app that requires a `iced::Element<AppMessage>` you have to **map** them.

For that, you can simply use `iced::Element<ViewMessage>.map(AppMessage::ViewMessage)`.
> **Note:** this is a shortcut for `iced::Element<ViewMessage>.map(|view_message| AppMessage::ViewMessage(view_message))`

The `.map` function takes a closure that takes the message and converts it into another message.
This often requires having a dedicated message variant on the application level that contains the view message.

Map functions like this are available on [`Task::map`](https://docs.iced.rs/iced/struct.Task.html#method.map), [`Subscription::map`](https://docs.iced.rs/iced/struct.Subscription.html#method.map), [`Element::map`](https://docs.iced.rs/iced/type.Element.html#method.map).

## Example
The example shows an unfinished joke listing app with a view for creating jokes.
When the user tries to create a new joke, a default joke, fetched from an API, is provided.
The user can get a random joke from the API on a button click as well.
All Jokes are listed in the main/default view.

This is how the view for new jokes looks like using this design:
```rust
{{#rustdoc_include app_structure_example/src/main.rs:new_view}}
```

As you see, the update function produces this action, that the parent of the view can handle:
```rust
{{#rustdoc_include app_structure_example/src/main.rs:action}}
```

The application that would host the view could look like this:
```rust
{{#rustdoc_include app_structure_example/src/main.rs:app}}
```
