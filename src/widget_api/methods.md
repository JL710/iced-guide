# Methods of the Widget trait

## State (child states), Tag and Diff

A widget can have a state or be stateless. A widget state can contain data that should live longer than the widget itself. A widget itself lives quite short. After each view call, it gets recreated. The scrollable, for example, saves the scroll position in its state. That way, the scroll position stays after each view call.

If your widget has a state, you need to implement the `tag` and `state` methods; otherwise, you can just use the default implementation. If you reuse other widgets that have a state, you are required to implement the `children` method.

### State method

At best, the state of your widget is created once and will be reused as much as possible. The `state` method returns the initial state of your widget if a new one needs to be created. It could look like this:
```rs
fn state(&self) -> iced::advanced::widget::tree::State {
	iced::advanced::widget::tree::State::new(YourCustomState::default())
}
```

### Tag method

To identify the different states, the `Tag` is used. The `tag` method returns a Tag based on your state type. Internally, it uses the TypeId of your state to identify it. 

> **Note:** You are right if you think: "If if uses the type ID, can two of my widget states be accidentally swapped?", you are right. They can. For those cases the diff method should fix it.

For most use cases, the `tag` method will look like this:
```rs
fn tag(&self) -> iced::advanced::widget::tree::Tag {
	iced::advanced::widget::tree::Tag::of::<YourCustomState>()
}
```

> If you have a widget that is stateless, you can just use the default implementation from the `Widget` trait.

### diff

This function compares/reconciles the old state `Tree`, with what you would expect it to be. If there should be something different, you change it in the given tree.  
There are two cases where this is needed. The first one is when two states with the same tag are accidentally swapped. In the second one, the state should change based on data passed to the widget in a view call.

If your widget has child widgets, it should contain something like this to diff the children:
```rs
fn diff(&self, tree: &mut Tree) {
	tree.diff_children(&self.children);
}
```

### children

If your widget uses other children, you need to return their state `Tree`s from the `children` method. The order of the child `Tree`s returned determines the order of `tree.children` in all other methods.

This method could look like this:
```rs
fn children(&self) -> Vec<Tree> {
	vec![Tree::new(&self.content)]
}
```

## size

The `size` method returns the size of the widget. That size can be used by other widgets to find a good layout. A good example of this is [`iced::advanced::layout::flex`](https://docs.rs/iced/0.14/iced/advanced/layout/flex/index.html).

The size method could look like this:
```rs
fn size(&self) -> iced::Size<iced::Length> {
    iced::Size::new(iced::Length::Shrink, iced::Length::Fill)
}
```

## layout

The `layout` method defines the layout of the widget and all of its child widgets. It returns a `Node` that represents the layout of the widget and all of its children.

To do that, you are given the `Limits` of the widget, meaning the minimum and maximum size that the widget can get, and the current state `Tree`.

If you have child widgets, you need to call their `layout` methods with their state from `tree.children`, the `Renderer` (you can simply use the one that is given as a parameter) and limits you want to assign to the child. The returned `Node` shall be included in the returned `node.children` of your widget.

## draw

The draw function uses the given `Renderer` to draw the widget onto the screen. The renderer can implement different `Renderer` traits.

With the given viewport, you can know what region of your window is currently visible.

In general, your widget should use the theme provided colors to fit into the application style. The text color is provided by the style parameter.

You can access the position, area and layout of your children via the given `Layout` parameter. The order of child layouts provided in `layout.children()` is equal to the order of `node.children` returned by the `layout` method.  
To draw child widgets, you can simply call their `draw` methods with the appropriate state (from `tree.children`) and layout (from `layout.children()`).

Since you might want to draw some special effect or graphic depending on the mouse position, you can access it if available with the `Cursor` parameter.

## operate

This function applies some given operation.
The `Operation` can have different effects.

You should apply all the appropriate functions of the `Operation` trait. To do so, you pass your state to the function and an Id that identifies your state.
Of course, you should only apply functions that are used. If your widget can not be focused, you should not call the `focusable` function.

If your widget has a child widget that does some operations, your operate method could look like this:
```rs
fn operate(
	&self,
	tree: &mut Tree,
	layout: Layout<'_>,
	renderer: &Renderer,
	operation: &mut dyn Operation,
) {
    operation.container(self.id.as_ref(), layout.bounds());
    operation.traverse(&mut |operation| {
        self.content.as_widget_mut().operate(
            tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    });
}
```

If you are interested in implementing operations for your widget, see the [operation section](./operations.md).

## update

The `update` method processes an `Event`.

The `Cursor` parameter provides access to the current mouse position.
The `Clipboard` parameter gives access to the clipboard of the user's system.

This is the only method of your widget that can emit messages to the application. For that, a `Shell` is provided as a parameter.
But the `Shell` can do other things as well. You can check if the current layout is valid or invalidate it, request redraws, check if the widget tree is valid and invalidate the widget tree, etc.

You must manually trigger redraws in update if your widget is meant to be interactive. You can take a look at [`Button`'s `update` method](https://github.com/iced-rs/iced/blob/0.14/widget/src/button.rs#L275-L361) to see how it reacts to mouse interactions.

You can also [*capture* events](https://docs.rs/iced/latest/iced/advanced/struct.Shell.html#method.is_event_captured) if you want to sign to ancestor widgets that they should ignore it, and check whether the event you received [is captured](https://docs.rs/iced/latest/iced/advanced/struct.Shell.html#method.is_event_captured) so you may ignore it yourself. This is optional, and widgets can choose to ignore that a certain event is captured, reacting to it all the same.

> **NOTE:** If you want to check for the event being captured by child widgets, make sure to call `update` on them first and *then* check whether the event is captured.

If you have child widgets that you want to produce messages that are local to your widget, just like a component, you can create a new `Shell` and give it to them in the `update` method.

If you want to do some animations, you can trigger/request redraws with the shell until the animation is over, possibly making use of the [`animation`](https://docs.rs/iced/0.14/iced/animation/) module.

## mouse_interaction

This method returns the current mouse `Interaction`.  
A mouse interaction is mostly how the cursor is displayed. You often see your cursor changes when you are resizing or moving some element on the screen.
If your widget, for example, has some area that can be grabbed and moved, you can return the `Interaction::Move` while the cursor is over that area to change the look of the cursor.

## overlay

This method returns the overlay of the widget if there is any. A good example of this can be found in the form of the tooltip widget.

If you have child widgets it could look something like this:
```rust
fn overlay<'b>(
    &'b mut self,
    tree: &'b mut Tree,
    layout: Layout<'b>,
    renderer: &Renderer,
    viewport: &Rectangle,
    translation: Vector,
) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
    overlay::from_children(
        &mut self.children,
        tree,
        layout,
        renderer,
        viewport,
        translation
    )
}
```
