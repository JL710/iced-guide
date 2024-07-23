# Debugging Wasm (ready)

If you encounter panics and errors in your browser console from your wasm builds, you might notice that these errors are nearly impossible to interpret.

To get a better error with a notice where exactly in your code the panic occurred, you have to change the panic hook to one that provides better exceptions in your browser.

For that, we can use the crate [`console_error_panic_hook`](https://crates.io/crates/console_error_panic_hook/).

To use that one you need to set the panic hook at the start of your program like here:

```rust,ignore
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // ...
}
```

With that in place, the errors in your web console should look a lot better.
