# Blocking Code

> **Note:** Currently, it is not possible to run blocking code on wasm.


## Tokio
If your iced application is using the [`Tokio`](https://docs.rs/tokio/latest/tokio/) runtime, to run blocking code in a `Task` or a `Subscription` we can use [`tokio::task::spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html).

### Example
Here is a small example that shows how to use [`tokio::task::spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html).

#### Cargo.toml
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

### Full Code
```rust
{{#rustdoc_include snippets/blocking_code.rs:all}}
```

## Smol
If your iced application is using the [`Smol`](https://docs.rs/smol) runtime, you can use [`smol::unblock`](https://docs.rs/smol/latest/smol/fn.unblock.html).

Add [`smol`](https://docs.rs/smol/latest/smol/) to cargo.toml:

```toml
smol = "2"
```

```rust
#[derive(Debug, Clone)]
struct MeaningOfLife(String);

async fn meaning_of_life() -> MeaningOfLife {
    smol::unblock(|| calculate_meaning()).await
}

fn calculate_meaning() -> MeaningOfLife {
    std::thread::sleep(Duration::from_millis(3000));
    
    MeaningOfLife(String::from("The meaning of life is 42."))
}


```
## Oneshot Channel

Another way to run blocking code is to use a [`oneshot`](https://docs.rs/futures/latest/futures/channel/oneshot/index.html) channel:

```rust
{{#rustdoc_include {{code}}/blocking-code/src/blocking_code_oneshot.rs:oneshot}}

```
A oneshot channel provides a type-safe way of sending a single value between threads / asynchronous tasks.

Furthermore, this approach does not require any extra dependencies or features as iced conveniently re-exports the [`futures`](https://docs.rs/futures/latest/futures) crate.

## Complete Example
```rust
{{#rustdoc_include {{code}}/blocking-code/src/blocking_code_oneshot.rs:all}}

```