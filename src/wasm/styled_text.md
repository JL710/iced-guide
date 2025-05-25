# Styled Text

When you first try to style text on wasm, like making text bold, you quickly notice that it does not work.

Why is that? Iced with default features, only loads `Fira Sans Regular` as a font.
This being a single weight font, limits the styling capabilities.
The advantage by only loading a very small font is to reduce the load time/size.

To fix that, you can simple download more fonts as you need from [github](https://github.com/mozilla/Fira/tree/master/ttf) and load them into your [Application](https://docs.rs/iced/latest/iced/application/struct.Application.html) by using the [font](https://docs.rs/iced/latest/iced/application/struct.Application.html#method.font) method like this:
```rust
.font(include_bytes!("FiraSans-Bold.ttf"))
```

> Since this is only necessary on wasm, if your app runs on native and wasm, you should only load it when compiling to wasm.
