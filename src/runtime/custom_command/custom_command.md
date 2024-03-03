# Custom Command using perform
In this example, we will create an app that fetches your current IP address by making an API call with our own custom command..

## Dependencies
As you see, we have two dependencies in our project. 

One of them is [reqwest](https://docs.rs/reqwest/latest/reqwest/index.html). We use reqwest to make the API call.

The other one is iced. 
Since this is a guide for iced, that should not wonder you. 
But as you see, we added the [`tokio`](https://docs.rs/crate/iced/latest/features#tokio) feature.
This lets iced use tokio as part of the runtime as needed for reqwest.

```toml
[dependencies]
iced = {version="0.12.1", features = ["tokio"]}
reqwest = "0.11.24"
```

## Making the api request
At first, we define what our command will do.

For that, we are creating an async function that makes an async get request to an API that provides the public IP.
```rust,ignore
{{#rustdoc_include snippets/main.rs:fetch_ip}}
```

> **Tip:** If you have something that is not async but synchronous and will block your application like a heavy computation,
> you can use [`tokio::spawn_blocking`](https://dtantsur.github.io/rust-openstack/tokio/task/fn.spawn_blocking.html) in a command or subscription to run a closure on a thread where blocking is acceptable.

## Starting/Creating the command
In the update function we return `Command::none()` or our custom command depending on the message.

If the Message is `Message::CurrentIp` we change our state, if it is `Message::Refetch` we return our command.
```rust,ignore
{{#rustdoc_include snippets/main.rs:update_function}}
```

To create our custom command, we use the [`Command::perform`](https://docs.rs/iced/latest/iced/command/struct.Command.html#method.perform) function.
It takes a future, in this case our `fetch_ip` function, and a closure that converts the returned value of the future into a massage.

```rust,ignore
{{#rustdoc_include snippets/main.rs:return_custom_command}}
```

> **Note:** `fetch_ip()` produces the future

> **Note:** `Message::CurrentIp` is a shorthand for `|x| Message::CurrentIp(x)`

## Full Code
```rust,ignore
{{#rustdoc_include snippets/main.rs:all}}
```