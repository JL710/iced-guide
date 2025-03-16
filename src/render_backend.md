# Render Backend

## Forcing Software Rendering:

By default, Iced tries to use wgpu as the backend, and if that is not possible, it uses tiny-skia as a fallback software renderer.

If you want to specifically use tiny-skia as render backend, you can do that with an environment variable `ICED_BACKEND = tiny-skia` :
```toml
[env]
ICED_BACKEND = "tiny-skia"
```

## Selecting a Graphics Backend for WGPU:

For Hardware Accelerated Rendering(i.e. Using a GPU to render) Iced uses [WGPU](https://github.com/gfx-rs/wgpu).

WGPU by itself can use various graphics API's like OpenGL, Vulkan, DirectX, Metal, etc.

To force a specific graphics backend for WGPU you can set an environemnt variable `WGPU_BACKEND = vulkan`:
```toml
[env]
WGPU_BACKEND = "vulkan" # It can have these values: dx12 / vulkan / metal / gl
```

> **NOTE**:
>
> To set these Environment Variables in Cargo, you need to create a `.cargo` directory in the root of your repository.
> Then create a `config.toml` in this directory and then add the code mentioned above.
>
> Example of `config.toml` file contents:
>
> ```toml
> [env]
> WGPU_BACKEND = "vulkan"
