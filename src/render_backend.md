# Render Backend

By default, Iced tries to use wgpu as the backend, and if that is not possible, tiny-skia.

If you want to specifically use tiny-skia as render backend, you can do that with an environment variable:
```
ICED_BACKEND=tiny-skia
```
