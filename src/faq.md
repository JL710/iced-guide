# Frequently Asked Questions

## Can I make a mobile app with iced?

Not really. There is a [discord thread](https://discord.com/channels/628993209984614400/1256812841944748133/1256812841944748133) for making an android app with some success in running an iced app on android. 
For IOS, there is [this repository](https://github.com/iced-rs/ios-examples), but it is marked as archived and the last commit is years old.

## Is there an easy way to create pop-ups?

Iced has no built in way for pop-ups such as error, ok/cancel and file dialog popups. Although you could build them by creating multiple windows, this can be a bit complicated at the beginning. A lot of people simply use [`rfd`](https://docs.rs/rfd/latest/rfd/index.html) for that use case, which works great, but has a GTK3 runtime dependency on Linux for its `MessageDialog`. Another option for message dialogs is [`iced_dialog`](https://iced-dialog.pml68.dev/iced_dialog/index.html), which provides an iced "native" implementation with no extra dependencies.

## How can I run stuff in the background / multithreaded / async?

You can do that using Subscriptions and Tasks. Take a look at the [Runtime section](./runtime/task_subscriptions.md) to learn more about them.

## Can I use iced from another languages?

Although I would not advise you to do so, there is a [python binding](https://github.com/icedpygui/IcedPyGui) and a [haskell wrapper](https://github.com/ibaryshnikov/iced-hs).

## When is the next release, and what features will it bring with it?

The next release will probably happen when the to-dos on the [roadmap](https://whimsical.com/roadmap-iced-7vhq6R35Lp3TmYH4WeYwLM) are finished.

## Serde support for iced types?

There was a [discussion on discord](https://discord.com/channels/628993209984614400/1304452091808583732/1304452091808583732) about adding serde support for the types in iced like `Theme` and `Color`. In the end, hecrj (maintainer of iced) was decided against it.
