# Tasks
> *Note:* in the past `Task`s where named `Command`s

A task is "A set of asynchronous actions to be performed by some runtime".

Basically, a task is just a [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) that returns messages.

You can create custom tasks, but often you get them by some function and just want to execute it. 
For example, [minimizing](https://docs.rs/iced/latest/iced/window/fn.minimize.html) and [maximizing](https://docs.rs/iced/latest/iced/window/fn.maximize.html) a window requires executing a given task.

A task will run until it has finished and can return multiple messages during its execution.

## Executing a Task
In your App, you can execute a task by returning it from the [update](https://docs.rs/iced/latest/iced/application/trait.Update.html) function of your application.

## Batch multiple tasks
Sometimes you want to return more than one task. 
For that, you can use the [Task::batch](https://docs.rs/iced/latest/iced/task/struct.Task.html#method.batch) function to batch a few of them together like this:
```rust
return Task::batch(vec![task1, task2, task3]);
```
