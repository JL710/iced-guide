# Component

> **Important Notice:** The Component Trait is deprecated, and I strongly advise against its use. If you are interested in creating reusable components, take a look at the [App Structure](./../app_structure/app_structure.md) section.

Sometimes you want to create your own reusable custom components that you can reuse through your applications.
That is where the [`Component`](https://docs.rs/iced/latest/iced/widget/trait.Component.html) trait comes in place. You can turn anything that implements this trait easily into an Element with the [`component`](https://docs.rs/iced/latest/iced/widget/fn.component.html) function.

Components should only be used as reusable widgets and not for organizing code or splitting your applications into different parts!

> **Note**: Component is only available on crate feature `lazy`.

## The 3 parts of a Component
Each component is build out of three parts: the component itself, the state of the component and the internal message of the component.

These are similar to the 3 parts of your application with one difference. The internal state that can change based on events is represented as an extra a struct, not as the component struct itself.

## Creating a Hyperlink Example
### Component itself
At first, we create the component struct itself:
```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:hyperlink_struct}}
```

As you see, it has one field `link`. Here, we store the link that will be displayed and opened when the hyperlink is clicked.

### Message / Event
Now we need to create the message that will be used inside our component:
```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:hyperlink_event}}
```
Here we have three events for our component. The `Clicked` event is called every time the user clicks onto the component. 
The `MouseEnter` and `MouseExit` events are called when the mouse enters over the component and leaves (in other words, hovering over the component).

### State
In the state of our component, we store if the mouse hovers over the component.
```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:hyperlink_state}}
```

### Implementing the Component trait
Now we can implement the [`Component`](https://docs.rs/iced/latest/iced/widget/trait.Component.html) trait for the `Hyperlink` struct.

#### Full Implementation
```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:implementing_component}}
```

#### Types
We define the types for our state and message/event:
```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:component_types}}
```

#### View and Update logic
Every time an event is called, the update and view function gets called.

In the update function, we set the hovered field of the state or print "open link". 
Instead of printing something you could use crates like [opener](https://docs.rs/opener/latest/opener/) to open files and website, but that is beyond this example.

```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:component_update}}
```

As you see, we return `None` in the update function. Instead of `None` we could return a `Some(Message)` that is propagated to the parent application.

We define in the `view` function how our component looks on the screen.
In this case, we have a mouse area with a text inside.

The text color changes when the mouse is hovered over the component. 
If the mouse hovers above the component is determined by the `state.hovered` field that is hold up to date by our update function.
```rust,ignore 
{{#rustdoc_include snippets/hyperlink.rs:component_view}}
```
