# Commands / Tasks

> **Note:** In the past releases of Iced there were Commands. In the current master branch and probably the next release, they are renamed to Task. Since this change this Guide uses sometimes Command and sometimes Task but meant the same thing.

A command is "A set of asynchronous actions to be performed by some runtime".

Basically, a command is just a [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) that returns messages.

You can create custom commands, but often you get them by some function and just want to execute it. 
For example, [minimizing](https://docs.rs/iced/latest/iced/window/fn.minimize.html) and [maximizing](https://docs.rs/iced/latest/iced/window/fn.maximize.html) a window requires executing a given command.

A command will run until it has finished and can return multiple messages during its execution.

## Executing a Command
In your App, you can execute a command by returning it from the [update](https://docs.rs/iced/latest/iced/application/trait.Application.html#tymethod.update) function of your [`Application`](https://docs.rs/iced/latest/iced/application/trait.Application.html).

> **Note:** This is not possible when you use [`Sandbox`](https://docs.rs/iced/latest/iced/trait.Sandbox.html) instead of Application  

## Batch multiple commands
Sometimes you want to return more than one command. 
For that, you can use the [Command::batch](https://docs.rs/iced/latest/iced/command/struct.Command.html#method.batch) function to batch a few of them together like this:
```rust,ignore
return Command::batch(vec![command1, command2, command3]);
```