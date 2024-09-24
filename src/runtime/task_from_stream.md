# Task From Stream
Imagine you want a `Task` that produces more than one `Message.` One solution is to use `Task::run` and pass a Stream to it.

A `Stream` is basically an async iterator.

To create a `Stream` we can use `iced::stream::channel`.
With that function, we can convert a `Future` to a `Stream`. In the `Future` we can emit messages via a given `Sender`.
