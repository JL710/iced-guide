
# The Component Pattern

The component pattern allows you to cleanly structure your application, including your state, update and view logic.
This pattern can also be found in this [showcase](https://github.com/hecrj/icebreaker) from the founder of iced.

Just like your top-level iced application, a component implements the Model-View-Update architecture and implements the following:

- The component state, usually named after its function (e.g. `LoginForm`)
- Its own `Message`
- An `Action` enum

In effect, the component is a self-contained iced program.

The state will have a normal view function that returns an [`iced::Element<Message>`](https://docs.rs/iced/0.13.1/iced/type.Element.html).

The update function will differ a bit. Instead of a Task like our main application, it will return an action enum.

```rust
impl MyComponent {
    // instead of this
    pub fn update(&mut self, message: Message) -> Task<Message> {}
    // we implement this
    pub fn update(&mut self, message: Message) -> Action {}
}
```

Common variants the `Action` enum can be `None`, `Task(iced::Task<Message>)` and `Submit { username: String, password: String }`. You can think of this action like a message that is sent from the view to the parent application that should handle it.

## The State

Your state is usually called after its function, e.g. `NewJoke` or `LazyImage`.
It contains the data you are currently working with.

```rust
{{#rustdoc_include {{code}}/app-structure/src/new_joke.rs:state}}
```

Your state should have a function that creates a new instance.

```rust
{{#rustdoc_include {{code}}/app-structure/src/new_joke.rs:new}}
```

### Handling lazy loading

If you want to lazily load some data from an API, or do something else in the background, you can use an enum to indicate the state.
You can either hold that enum as part of your State struct, or just use the enum directly.

```rust
pub enum LazyImage {
    Loading,
    Loaded {
        title: String,
        image: Vec<u8>,
    },
    Error(String),
}
```

If you need to execute some code asynchronously after your component is created, you can instead return `(Self, iced::Task<Message>)`.
This mirrors iced's `run_with` function, which allows you to run a task when starting your application.

```rust
impl LazyImage {
    pub fn new(url: String) -> (Self, iced::Task<Message>) {
        let task = iced::Task::spawn(async move {
            // load your image data
            let response = todo!().await;
            match image {
                Ok(data) => Message::Loaded {
                    title: data.title,
                    image: data.body
                },
                Err(error) => Message::Error(error.to_string())
            }
        });

        (Self::Loading, task)
    }
}
```

## Message

Your component will have its own internal messages, which work just like in your top-level iced application.
For a `LoginForm` they might look like this:

```rust
{{#rustdoc_include {{code}}/app-structure/src/new_joke.rs:message}}
```

## Embedding our state

To use our component, we'll need to add it to our application state.

Depending on your use case, you can embed it directly, as an `Option` or inside another enum.
For demonstration purposes, we'll add our own little enum.

```rust
{{#rustdoc_include {{code}}/app-structure/src/main.rs:view_enum}}
```

Now that we have a nice way to specify out view, let's add it to the apps state:

```rust
{{#rustdoc_include {{code}}/app-structure/src/main.rs:app_state}}
```

Before we can enjoy our beautiful component, we'll need to actually create the component's state somewhere.
In our case, we want to show it, after the user clicks an "Add Joke" button.

For that we'll just add a button to the app's `view` method and edit the app's `update` function to include this:

```rust
impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
{{#include {{code}}/app-structure/src/main.rs:create_component}}
            // ...
        }
    }
}
```

## View & Mapping

Following the trend of mirroring what an iced application does, you'll also want to implement a `view` function.

```rust
{{#rustdoc_include {{code}}/app-structure/src/new_joke.rs:view}}
```

Now we'll obviously want to use this view as part of our main view.
But our main view expects a return value of `iced::Element<crate::Message>`, while our view returns `iced::Element<new_joke__message>`.
Thankfully, iced allows us to **map** them.

First we'll need to add a message variant to our main `Message`.

```rust
// Main Application Message
pub enum Message
{
    NewJoke(new_joke::Message),
    // ...
}
```

After that we can actually call the `view` method of our new component.

To map our Message, we can simply use `iced::Element<component::Message>.map(crate::Message::Component)`.
> **Note:** this is a shortcut for `iced::Element<component::Message>.map(|component_message| crate::Message::Component(component_message))`


```rust
impl App {
    fn view(&self) -> iced::Element<Message> {
        match &self.view {
{{#include {{code}}/app-structure/src/main.rs:new_joke_view}}
            // ...
        }
    }
}
```

The `.map` function takes a closure that takes the message and converts it into another message.
This often requires having a dedicated message variant on the application level that contains the view message.

Map functions like this are available on [`Task::map`](https://docs.iced.rs/iced/struct.Task.html#method.map), [`Subscription::map`](https://docs.iced.rs/iced/struct.Subscription.html#method.map), [`Element::map`](https://docs.iced.rs/iced/type.Element.html#method.map).

## Update & Action

As already hinted in the beginning, the update function of a component does have a significant change compared to a normal iced application.

Instead of returning an `iced::Task`, we return an `Action`.
An `Action` allows us to communicate with the parent of our component. In that regard, they are similar to events from other UI frameworks.

Some applications, like [Halloy](https://github.com/squidowl/halloy) actually do call this type `Event` instead of `Action`.

First we'll start by defining our component's action type:

```rust
{{#rustdoc_include {{code}}/app-structure/src/new_joke.rs:action}}
```

With our action ready, we can add our update function. To make the update method easier to read, we handle task creation in a seperate function.

```rust
{{#rustdoc_include {{code}}/app-structure/src/new_joke.rs:update}}
```

As with the view before, we'll now need to call our component from the app's main update function.
Our component's update function now returns an `Action` that we'll want to react to.

> **Note:** As with our view before, we'll have to map the task, should one be returned.

```rust
impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
{{#include {{code}}/app-structure/src/main.rs:update_component}}
            // ...
        }
    }
}
```

After hooking up the `view` and `update` functions, we're done.

## Conclusion

The component pattern is the default way to divide your state and update logic into smaller pieces.
Keep in mind that default doesn't necessarily mean it's the right solution for you.

This pattern is great because it's structured just like your iced application
and encompasses everything you need for that part of the application.

It does, however, introduce a lot of boilerplate code, which isn't always warranted.

If you don't need the internal state and update logic, you might instead be more interested
in the [Viewable Pattern](./viewable.md).
