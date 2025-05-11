# Structuring Apps
When you create larger iced apps, you might want to have reusable components and views.
There are a lot of different ways to structure your app.

To make starting out easier, you'll find some patterns you can use here.

> **NOTE:** These patterns are just suggestions for how you could solve the problem.
>
> Don't just follow them blindly and instead look at them more as examples on how you could solve the problem.
>
> You can mix and match these patterns to fit your need or build something entirely different.

To demonstrate our patterns, we'll build a simple joke listing app throughout this chapter.
You can view the resulting application here:

[https://github.com/JL710/iced-guide/tree/main/src/app_structure/app_structure_example]()


## Proposed Naming Convention

To support productive discussions and clearer explanations, this guide proposes the following pattern names.
These patterns have existed way before this article, but by giving them names, we can e.g. ask about the difference between a Viewable and a Widget?.

The names are in no way official or endorsed by the iced project.

### Widget

Before jumping into the Patterns, you should know that iced itself has a `Widget` trait,
which is used to implement the default widgets like `button` or `row`.

While a widget is the most versatile way to draw something on the screen or grab input,
it is significantly more complex and difficult to implement.

So before trying to implement a custom widget, we'd recommend you try one of the following patters first.

### View-Helper

A view-helper is the simplest way to add some composability to your application.
It's just a function, which creates an `Element`.

### Viewable

If you expand a view-helper, you might arrive at the viewable pattern.

A viewable is the little brother of a widget.
It allows you to enjoy the reusability of a widget without the complexity.

A viewable *does not* contain any application state itself.
It is instead used in building the view from your application state.

If you want to add state or interactions, take a look at the component pattern

### Component

> **WARNING:**
>
> Iced has a deprecated trait which is also called `Component`.
>
> The component pattern is something else and we strongly recommend using the component pattern instead of the old trait.

A component is a rust module, which can be used to implement an interactive part of your application.

Components can hold their own state and be interactive on their own.
