# Blocking Code
To run non async code / blocking in a `Task` or a `Subscription` we can use [`tokio::task::spawn_blocking`](https://dtantsur.github.io/rust-openstack/tokio/task/fn.spawn_blocking.html)

> **Note:** This might only work on native and not on wasm

## Example
Here is a small example that shows how to use [`tokio::task::spawn_blocking`](https://dtantsur.github.io/rust-openstack/tokio/task/fn.spawn_blocking.html).

### Cargo.toml
Because we want to use `spawn_blocking` from tokio we need to add the `tokio` feature to iced. This will lead to iced using tokio.
```toml
iced = { ... , features = ["tokio", ...] }
```

### Actual Example
In the example, there will be a button and a text. A press onto the button will trigger a large computation to be started (in the example, we will just sleep a few seconds and return a number). 
If the computation finishes, the result will be shown in the text widget.

Our computation runs in a task, because we do not want to block our whole UI until it has finished.

Inside the task we call `spawn_blocking` with a closure of our computation. To get the returned value of the closure, we need to await the `JoinHandle`  returned by `spawn_blocking`.
That will give us the result of the heavy computation without blocking the UI.

```rust
{{#rustdoc_include {{code}}/blocking-code/src/main.rs:compute_task}}
```

#### Full Code
```rust
{{#rustdoc_include {{code}}/blocking-code/src/main.rs:all}}
```