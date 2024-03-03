# A minimal Application - Counter

Our goal is to create a simple counter where we have a number displayed that we can increase or decrease with two buttons.

<div align="center">
    <img src="assets/counter-app-ss.png">
</div>

## Creating a new Project 
First of all, make sure *Rust* is installed in your system. If not head over to [Rust Installation Page](https://www.rust-lang.org/tools/install).

After installing rust, create a new binary crate by executing,
```console
$ cargo new counter-app-iced
$ cd counter-app-iced
```

Add Iced crate by executing,
```console
$ cargo add iced
```

Now, build the app using
```console
$ cargo run
```

On successful build, you can see the text `Hello World` is printed on console. Now we are ready to create our beautiful GUIs using Iced.

## 1. Defining the State
For the state, we define a struct. For the counter, we need to store the current value of the counter.
```rust,ignore
{{#rustdoc_include snippets/main.rs:counter_struct}}
```

## 2. Defining the Message
For our counter, we have two major events that matter to us. Increasing and decreasing the counter.

The message is represented as an enum with two variants, `IncrementCounter` and `DecrementCounter`.

```rust,ignore
{{#rustdoc_include snippets/main.rs:message_enum}}
```

## 3. Implementing the Sandbox for Counter
To create a window, we implement the [`Sandbox`](https://docs.rs/iced/latest/iced/trait.Sandbox.html) trait for our `Counter`. There are two different windowing implementations. One is `Sandbox` and the other is [`Application`](https://docs.rs/iced/latest/iced/application/trait.Application.html). The difference between both is that `Sandbox` trait provides a much simpler interface to work with if you are just getting started.

It's not harder to switch your app from `Sandbox` to `Application`. We will use the `Sandbox` trait throughout this section.

The `Sandbox` trait implments the following,

```rust,ignore
pub trait Sandbox {
    type Message: Debug + Send;

    // Required methods
    fn new() -> Self;
    fn title(&self) -> String;
    fn update(&mut self, message: Self::Message);
    fn view(&self) -> Element<'_, Self::Message>;

    // Provided methods
    fn theme(&self) -> Theme { ... }
    fn style(&self) -> Application { ... }
    fn scale_factor(&self) -> f64 { ... }
    fn run(settings: Settings<()>) -> Result<(), Error>
       where Self: 'static + Sized { ... }
}
```


### Message
The `Sandbox` trait expects us to define our own `Message` type. So we should map our `Message` enum as a type alias for `Message` in the `Sandbox` implementation. 
```rust,ignore
{{#rustdoc_include snippets/main.rs:counter_message_type}}
```

### Initial state and Title
Next, we need to set out initial value of our **state**, which is the `count` value. The `new()` function helps us to do exactly that. The state of the `Counter` is returned with it's initial `count` value as `0`.
```rust,ignore
{{#rustdoc_include snippets/main.rs:new}}
``` 

In, the `title()` function, we set the title of the app. This will be the title of the current instance of the Application.
```rust,ignore
{{#rustdoc_include snippets/main.rs:title}}
```

### Update logic
Now we have to handle the **messages** that are emitted by the View Logic. The `update()` function does exactly that. The `update()` function get called every time when the View Logic emits a message. We use the rust's powerful [`match`](https://doc.rust-lang.org/std/keyword.match.html) expression to handle messages. Here we use the `match` expression to increase the count when `IncrementCount` is emitted and decrease the count when `DecrementCount` is emitted.
```rust,ignore
{{#rustdoc_include snippets/main.rs:update}}
```
> **Tip:** Use `count.saturating_add(1)` or `count.saturating_sub(1)` for more error proof and optimized code.

### View logic
The only thing left is to define our View (a.k.a UI). Define your View Logic in the `view()` function. In iced, all UI components are called [**widgets**](https://docs.rs/iced/latest/iced/widget/index.html). For a counter, we need two `button` widgets (one for incrementing and another for decrementing) and a `text` widget. They need to be aligned one after another in a **horizontal** manner. So, we use `row` widget macro to align our widgets in a horizontal manner.
```rust,ignore
{{#rustdoc_include snippets/main.rs:view}}
```
In the above code, we can see that the `button`'s `on_press` function accepts the message type to be emitted.

> Note: By default, the `view()` function returns the type `Element<'_, Self::Message>`. So, we use `.into()` for conversion purpose.

That's pretty much everything for a simple counter app. Now, let's run it.

## 4. Running the Counter
Now we can run our simple Counter App by calling `Counter::run()` in the `main()` function. The `run()` method is a default implementation of the [`Sandbox`](https://docs.rs/iced/latest/iced/trait.Sandbox.html) trait.  
```rust,ignore
{{#rustdoc_include snippets/main.rs:main}}
```

> **Note:** The main function should have a return type of `Result<(), iced::Error>`.

## 5. Full Code
Now that we completed our simple counter application, the complete code will look like this.

```rust,ignore
{{#rustdoc_include snippets/main.rs:all}}
```

&nbsp;

In the next section, you will see about common layouting techniques that iced offers.