# Common Structs

## Limits

The limits contain the maximum space that the widget can take in the layout.

They contain the minimum limit that describes the space that the widget will get either way. So even if the given node is smaller, the space will be only used for that widget.

If the widget with shrink as width is placed in a row with an item that has set its size to fill, the minimum limit will be 0. If both widths are set to Fill, it will get the minimum limit of half the space that the row takes.

If the Size of the returned node is larger than the maximum specified by the limits, as far as I tested in rows and columns, other widgets that come after the widget will have just less space available (in their layout methods). But it is not recommended to take more space than there is specified as the maximum in the limits.

## Node's

A Node is a rectangle with position and size that can have a list of children.

You can move or translate its position with the `.move_to` and `.translate` position. The node also provides a lot of other helper methods that are worth looking at (i.e., padding).

## Tree's

A tree with the current state of the widget and all the states of its children.
