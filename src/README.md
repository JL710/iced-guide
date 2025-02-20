<div align="center">
    <img src="iced-logo.svg">
</div>

# Welcome to the unofficial iced-rs guide 

Iced is an Open Source GUI library written in rust to create beautiful and minimal cross platform applications. It leverages the power of Google's [Skia](https://skia.org/) (via [tiny-skia](https://github.com/RazrFalcon/tiny-skia)) and wGPU to render beautiful UI while maintaining clean and maintainable code by using the ELM (or MVU) architecture.

Iced focuses on Simplicity and type-safety so that you can concenterate on your implementation without any framework specific distractions.

This guide tries to explain the basics of the Iced GUI library for the Rust programming language.

## Disclaimer

To make it clear at the beginning. This is not an official guide. It is not approved by the iced-rs team!
If you search for the [official documentation](https://docs.rs/iced/latest/iced/) or [iced book](https://book.iced.rs/) take a look at the [Iced website](https://iced.rs/).

## Different versions

Since iced develops quite fast you might encounter big breaking changes in different versions including between the latest version and the master branch (dev version).
A lot of people on the [discord](https://discord.gg/3xZJ65GAhd) use the master branch and talk about features in it, so keep that in mind if you do not find them in the latest release.

## Documentation Resources

You will find docs of the released iced versions here on [docs.rs](https://docs.rs/iced/latest/iced/). 
If you want to see the docs of the master branch (dev version) you can get them [here](https://docs.iced.rs/iced/).
There are a lot of small examples available in the example's directory of the iced repo. But make sure you select the proper git tag/branch for the iced version that you use.

Since the last version, there is also a new, great [pocket guide](https://docs.iced.rs/iced/#the-pocket-guide) that I advise everyone to read.

## Other Resources

There are multiple guides and tutorials that help you to learn iced, including this guide and the [official book](https://book.iced.rs/).
Here is a list of other cool resources that you might find helpful:
- [awesome-iced](https://github.com/iced-rs/awesome-iced) a list of applications that use iced
- [Github Markdown Tutorial](https://github.com/fogarecious/iced_tutorial/tree/main) A very large tutorial covering a lot of stuff, but a bit out of date.
- [Youtube Text Editor Tutorial](https://www.youtube.com/watch?v=gcBJ7cPSALo): This is a tutorial on how to build a text editor with iced 0.10, so pretty out of date. It is still a good video tutorial, but a lot of the stuff won't work in the latest iced version and can be done way easier with newer versions.

If you search for additional widgets, you might be happy to see that there is [`iced_aw`](https://github.com/iced-rs/iced_aw), which provides additional widgets.

## Contribution

If you want to contribute to this guide, you can open an issue on GitHub and make a pull request. 
For large changes, it is preferred that you open an issue first to discuss the changes. 
For any small changes, spelling, grammar and formatting fixes, directly opening a pull request should not be a problem.

&copy; Héctor Ramón ([hecrj](https://github.com/hecrj)) for the iced logo.