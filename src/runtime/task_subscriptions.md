# Subscriptions and Tasks

Sometimes you have a task that takes a bit and should run in the background. If you just put it in your update function, the GUI will be locked and unresponsive until the task is finished. 
This might be a web request or listening to external events.

Iced has two solutions to this issue. The `Task` and `Subscription`.

A task will run until it is finished, while a subscription runs as long as the application says so.

In this chapter, we will look into both of them and how we can use them.
