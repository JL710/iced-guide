# Wasm / Running on Web
Iced has the ability to compile to web assembly (WASM). 
With that, you can run your iced app in the normal web browser.

## Preparation
Before we start, we need to get the tools for compiling for the web.

### Add Wasm Target
First, we need to add the `wasm32-unknown-unknown` target to our compiler so that we are able to compile to wasm.
```
rustup target add wasm32-unknown-unknown
```
If you want, you can compile to that target using
```
cargo build --target wasm32-unknown-unknown
```

### Install Trunk
We will use [trunk](https://trunkrs.dev/) for building and serving the web page. 
For that, we need to install trunk via cargo:
```
cargo install --locked trunk
```

### Create index.html
For trunk we need to create a `index.html` file that trunk will use as the base file.
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>NAME OF THE WEBSITE</title>
    <base data-trunk-public-url />
</head>
<body style="margin: 0; padding: 0; width:100%; height:100svh">
<link data-trunk rel="rust" href="Cargo.toml" data-wasm-opt="z" data-bin="directoryname" />
</body>
</html>
```
Replace the `directoryname` with the name of your project.

## Iced Features that are important for wasm
- `web-colors` on  the web, the colors of your app might not be correct or as intended, this feature fixes that.
- `webgl` makes your app run in browsers that do not support wgpu (sadly, wgpu is not supported on all browsers and all platforms).

## Running the App using Trunk
```
trunk serve
```
This will compile your project to wasm, build a web page, watch for changes and serve the app for you. You can access the web app at the URL given in the output.
The generated site that is served is located in the `dist` directory.

## Deploying the Web app
For now, we only used `trunk serve` to run our app. 
This works fine for development, but is not very good for production.
We don't need to watch for changes in our files and rebuild/autoload them in production. Without that functionality, the served html can be way smaller.

Thankfully, we can use `trunk build` to build our app into a minimal result. 
If you look at the `dist` directory, you can see your build app (`index.html` as the starting point).
> **Disclaimer:** If you try to open the index.html file without a web server that serves it, you will run into CORS issues.

You can serve these files on any web server. 
As a minimal example, you can use the built-in python web server and execute `python3 -m http.server` in the `dist` directory.
