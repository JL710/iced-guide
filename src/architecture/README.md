# Architecture
The architecture of iced is inspired by the [elm architecture](https://guide.elm-lang.org/architecture/).

In this architecture the GUI is splittet into four different parts:
- Message
- Sate
- Update Logic
- View Logic

## Message
The message defines any events or interactions that your program will care about. 
It will be implemented using the rust enum.

## State
The state contains all the data that your program wants to store throughout its lifespan. This is implemented using a struct.

## Update Logic
The update logic is called every time a message is emitted and can operate based on this message. This logic is the only one that can change the state of your application.

## View Logic
The view logic generates the view, elements/widgets, and layout based on the current state. The view logic is called every time after the update logic is called.