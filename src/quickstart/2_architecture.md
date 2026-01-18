# Architecture
The architecture of iced is inspired by the [elm architecture](https://guide.elm-lang.org/architecture/).
This architecture splits your code into 4 main parts:

- Messages
- State
- Update Logic
- View Logic

&nbsp;

<div align="center">
    <img src="assets/elm-schematic.svg">
</div>

> **NOTE:** The snippets shown below are just for example purposes and will not compile.

## State
The state contains all the data that your program wants to store throughout its lifespan. This is implemented using a struct. For example, in case of a simple counter app, which increments or decrements the current count value, the state would be like this,

```rust
struct Counter {
    count: i32
}
```
In the above snippet, all we need is a `count` value for a simple counter application. hence the state.

## Message
The message defines any events or interactions that your program will care about. 
In iced, it will be implemented using the rust enum. For example, let's take a simple counter app, the Messages / Events that might occur are stored in the Message enum, For example, 

```rust
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementCount,
    DecrementCount
}
```

## Update Logic
The update logic is called every time a message is emitted and can operate based on this message. This logic is the only one that can change the state of your application. A rough example of update logic with respect to the previous counter example is below,

```rust
fn update(&mut self, message: Message) -> iced::Task<Message> {
    match message {
        Message::IncrementCount => self.count += 1,
        Message::DecrementCount => self.count -= 1
    }

    iced::Task::none()
}
```

## View Logic
The view logic generates the view, elements/widgets, and layout based on the current state. The view logic is called every time after the update logic is called. So for a simple counter app, all we need is a `text` view and two `button`s. We can declare our UI as follows,

```rust
fn view(&self) {
    let ui = column![
        button("+").on_press(Message::IncrementCount),
        text(self.count),
        button("-").on_press(Message::DecrementCount)
    ]
}
```

Now that we got a basic understanding of the ELM architecture, we can deep dive into Iced and create a simple **counter** app.
