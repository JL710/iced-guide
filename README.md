# iced-guide

Welcome to the [Unofficial Iced Guide](https://jl710.github.io/iced-guide/)!

This guide is built using [mdBook](https://github.com/rust-lang/mdBook) ([mdBook Guide](https://rust-lang.github.io/mdBook/)). 
It is made up of basic markdown files with some extensions that mdBook provides ([mainly including file sections](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files)).

## File Layout

The markdown files are located in the `src` directory.
The `SUMMARY.md` dictates the layout of the guide, and all pages are registered here.

The file/directory layout is not required to match the guide layout by mdBook, but it is best practice to keep them somewhat in sync. 

Example code that is not directly in the markdown files is contained in the [iced-guide-examples](./iced-guide-examples) directory. You can reference that directory from anywhere in the guide by using `{{code}}/`.

## Run the guide for development

```
pip install git+git+https://github.com/JL710/mdbook-project-path.git
mdbook watch
```
> Note: You must have mdBook installed (`cargo install mdbook`).
