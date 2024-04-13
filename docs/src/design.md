# Design

## How's it work?

Each place a developer can "draw" to (render targets) implement some "Surface0" to read the "Scaffold" to be built and execute commands against the rendering backend however necessary.

In the case of Bevy and the Web, a Surface is "reactive" in the sense that it can re-draw the same set of "computed elements" (Entities and DOM Nodes, respectively) every time the `Surface` is drawn to.

Enables improved composition patterns while still allowing Bevy and the browser to use their respective APIs for things like state management, event management, routing, etc.

### Bevy

For Bevy, we continue to use Systems to listen for interaction events, draw to the UI/Scene/whatever, and listen for changes to `Resources`.

Events are forwarded from Bevy's event registry to the surface and assigned handlers are executed.

Most of the rest is 

### Web

For the web, we use (TODO)
