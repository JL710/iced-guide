# Structuring Apps
When you create larger iced apps, you might want to have reusable components and views.
There are several techniques to achieve this.

To demonstrate them, we'll build a simple joke listing app throughout this chapter.

## Proposed Naming Convention

To support productive discussions and clearer explanations, this guide proposes the following pattern names.
These patterns have existed way before this article, but the goal is to name and explain them.

The names are in no way official or endorsed by the iced project.

### Widget

Before jumping into the Patterns, you should know that iced itself has a `Widget` trait,
which is used to implement the default widgets like `button` or `row`.

While a widget is the most versatile way to draw something on the screen or grab input,
it is significantly more complex and difficult to implement.

So before trying to implement a custom widget, we'd recommend you try one of the following patters first.

### Viewable

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
