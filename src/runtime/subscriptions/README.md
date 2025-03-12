# Subscriptions

A subscription is similar to a task. It runs in the background. Subscriptions are often used to listen to external events. It can produce one or more values.
One key difference is that we control how long a subscription runs. That leads to the "issue" that the subscription itself can never end by itself, even after finishing its work.

> **Warning:** I am not that familiar with the iced internals so the following might be incorrect. It is only how I understood it.

A `Subscription` runs as long as we return it from the closure provided by the [`subscription`](https://docs.rs/iced/latest/iced/application/struct.Application.html#method.subscription) function.
The runtime calls that method after each update and checks if a new or old subscription is provided.

Every `Subscription` has an ID. If you use `Subscription::run_with_id` you specify the ID. If you use `Subscription::run` the function pointer is used, which could cause bugs, from what I have heard.

If a new subscription is provided, the runtime will start it. If an old one that already runs is provided, nothing happens. If a subscription runs that is not provided by the subscription function, the running subscription is terminated.

## Create a Stream
To create a `Stream` we can use `iced::stream::channel`.
With that function, we can convert a `Future` to a `Stream`. In the future, we can emit messages via a given `Sender`.