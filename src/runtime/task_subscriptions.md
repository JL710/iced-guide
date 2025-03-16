# Subscriptions and Tasks

Sometimes you have a task that takes some time to complete and should run in the background.
If you run it in your update function, the GUI becomes locked and unresponsive until the task is finished.
This could be a web request or an operation that listens for external events.

Iced offers two solutions to this issue: `Task` and `Subscription`.

A task runs until it completes, whereas a subscription continues running as long as the application requires it.

In this chapter, we will examine both solutions and explore how to use them.
