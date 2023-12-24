#![feature(allocator_api)]

use std::process::ExitCode;

use anyhow::Result;

// use slate::surface::SurfaceError;
// use slate::scaffold::Scaffold;
// use slate::element::Element;
// use slate::element::UUID;
use slate::element::tests::ElementTestImpl;

use terminal_slate::TerminalSurface;

//---
fn main() -> Result<ExitCode> {
    // TODO: Move this into the terminal config.
    #[cfg(feature = "raw")]
    slate::log::init();

    // 3. Render the tracing data to the screen
    tracing::info!("Hello, world!");
    
    // TODO: When we've implemented the phase-based render system,
    //   swap this out for Bumpalo.
    let mut terminal = TerminalSurface::new();
    
    // Support inline-events (when the renderer does).
    let _ = |evt: &OnLoadEvent| {
        println!("Clicked: {:?}", evt);
    };
    
    terminal.draw(chizel::uix! {
        // Some user-docs for this element.
        // TODO: Support doc-embedded commands?
        // #[on:load(load_callback)] // Use existing methods/clousures ..
        // #[cfg(ctx, show=self.debug_mode)]
        <ElementTestImpl
            // Each prop is transformed to a call, `.with_[key](value)`,
            // which takes an argument `Into<T>` for convenience.
            name="Test Element"
            
            // TODO
            number=0
        >
            // .. or define them inline.
            // #[on:click(|evt: &OnClickEvent| println!("Clicked: {0:}", evt))>]
            <ElementTestImpl name="Child of First Root" number=3>
                // Elements can be nested pretty far.
                <ElementTestImpl
                    name="First Nested Child of First Root"
                    number=10
                />
                <ElementTestImpl
                    name="Second Nested Child of First Root"
                    number=31
                />
            </ElementTestImpl>
        </ElementTestImpl>
        
        // Multiple roots are parsed into a grouped node.
        <ElementTestImpl name="Second Root">
            // Setup a child node.
            <ElementTestImpl name="First Child of Second Root" />
            <ElementTestImpl name="Second Child of Second Root" />
            <ElementTestImpl name="Third Child of Second Root" number=6>
                // Elements can be nested pretty far.
                <ElementTestImpl name="First Nested Child of Third Child of Second Root" />
            </ElementTestImpl>
        </ElementTestImpl>
    })?;
    
    terminal.start()?;

    Ok(ExitCode::SUCCESS)
}

//---
/// TODO
#[derive(Default, Debug)]
pub struct OnLoadEvent;

/// TODO
#[derive(Default, Debug)]
pub struct OnClickEvent;
