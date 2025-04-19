# Operations

Operations can query and update a widget state.

An operation provides specific functions defined in the `Operation` trait. These functions take the state of the widget and an identifier to locate the relevant state/widget.
For common operations like focusable and text editing widgets there are fixed functions in the `Operation` trait. These functions require the states to implement unified traits such as `Focusable` and `TextInput`.
If you want some custom operation that is unique to your widget, you can do that with the custom function of the `Operation` trait. 

These functions are provided to the widgets, and it is their job to apply them in their `operate` function. 

To launch/start/emit and `Operation` you will need to send a Task to the runtime. You can create the Task using the helper function `operate`.

## Custom Operation

If you want to create a custom Operation it is convention to create a trait for it. All the widget states that the operation can be applied to should implement this trait.

In your implementation of the `Operation` trait, you will need to implement the custom method.
This function takes a dynamic type `Any` that will be the state and an Id that identifies the widget state.
You can downcast the state to a trait object that matches the previous defined trait that your state implements.
After downcasting the state you can use the methods provided by your trait to modify the state.
