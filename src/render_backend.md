# Render Backend

## Forcing Software Rendering:

By default, Iced tries to use wgpu as the backend, and if that is not possible, it uses tiny-skia as a fallback software renderer.

If you want to specifically use tiny-skia as render backend, you can do that temporarily by setting the `ICED_BACKEND` environment variable to `tiny-skia`, e.g.
```sh
ICED_BACKEND=tiny-skia cargo run
```

A more permanent solution can be achieved by disabling default features and re-enabling everything you need except for `wgpu`:
```toml
[dependencies]
iced = { version = "0.14", default-features = false, features = [
    "tiny-skia",
    "x11",
    "wayland",
    "smol",
    "image"
] }
```

## Selecting a Graphics Backend for WGPU:

For Hardware Accelerated Rendering(i.e. Using a GPU to render) Iced uses [WGPU](https://github.com/gfx-rs/wgpu).

WGPU by itself can use various graphics APIs like OpenGL, Vulkan, DirectX, Metal, etc.

To force a specific graphics backend for WGPU you can set an environemnt variable `WGPU_BACKEND = vulkan`:
```toml
# /path/to/project/.cargo/config.toml

[env]
WGPU_BACKEND = "vulkan" # It can have these values: dx12 / vulkan / metal / gl
```
