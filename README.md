# Slate

Various Rendering systems like Bevy, Ratatui, and the web hold a Surface, which itself holds a collection of Scaffolds and a Styleguide.

## Workspace

### Slate

The primary/root crate. Provides structures for building and updating user-interfaces. Primarily provides composition and reactivity.

-   A `Surface` struct acts as a reactive core for various runtimes which render composed UI such as Ratatui, Bevy, and the web (via WASM).

-   A `Scaffold` struct which holds a UUID and a 

### Chizel

A set of macros used to compose interfaces with a JSX-like DSL. Attributes are parsed to adorn composed elements with styles, event handlers, etc.
    
### Derive Example:

```rust
#[derive(Element)]
struct TextInput {
    #[prop(value)]
    value: Option<Mutex<String>>,
    cursor_pos: usize, // No prop ..
    // etc..
}

impl Element for TextInput {
    pub fn render<C: Element>(&self, children: &[C]) -> DrawFn {
        
    }
}
```

### UIx Example:

```rust
// Note: Draw takes a `DrawFn`, which is composed by the `uix!` macro.
surface.draw(chizel::uix! {
    // Events can be registered by passing any `EventHandlerFn` as an
    // event attribute, which is prefixed as `on:[event-name]`.
    #[on:load(load_callback)]
    // In this case, we take two existing class names (any expression)
    // and any number of key=value pairs, where the key is an Ident
    // and the value can be any expression.
    // any expression.
    #[style(class01, class02, render=self.debug_mode)]
    <ElementTestImpl
        // Each prop is transformed to a call, `.with_[key](value)`,
        // which takes an argument `Into<T>` for convenience.
        name=root01_name
        number=0usize
        // Wrap complex types in curly bois (for now).
        uuid={UUID::new_v4()}>
        // .. or define them inline.
        // #[on:click(|evt: &OnClickEvent| println!("Clicked: {0:}", evt))>]
        // #[style(background-color=hexa("ff0000", 0.5))]
        <ElementTestImpl name="Child of First Root" number=3>
            // Elements can be nested pretty far.
            <ElementTestImpl name="First Nested Child of First Root" number=10 />
            <ElementTestImpl name="Second Nested Child of First Root" number=31 />
        </ElementTestImpl>
    </ElementTestImpl>

    <ElementTestImpl name="Second Root">
        <ElementTestImpl name="First Nested Child of First Child of Second Root" />
    </ElementTestImpl>
    
    // Multiple roots are parsed into a grouped node.
    <ElementTestImpl name="Third Root">
                // Setup a child node.
        <ElementTestImpl name="First Child of Third Root" />
        <ElementTestImpl name="Second Child of Third Root" />
        <ElementTestImpl name="Third Child of Third Root" number=6>
            // Elements can be nested pretty far.
            <ElementTestImpl name="First Nested Child of Third Child of Third Root" />
        </ElementTestImpl>
    </ElementTestImpl>
});
```