use std::process::ExitCode;

use anyhow::Result;

use slate::scaffold::Scaffold;
use slate::element::tests::ElementTestImpl;
use slate::event::ClickEvent;
use slate::event::EventPin;

//--
/// A wimple example which builds a Scaffold from basic UIx markup.
/// Hint: Use with `cargo expand` to view generated composition code.
/// Example: cargo expand [-p slate] --example slate-bones
fn main() -> Result<ExitCode> {
    slate::log::init("trace");
    
    #[cfg(feature = "bump")]
    let arena = slate::arena::get();
    
    let scaffold = Scaffold::try_from_draw_fn(
        #[cfg(feature = "bump")]
        arena.as_bump(),
        {
            let on_click_fn = |_: &ClickEvent| {
                println!("Clicked!");
            };
            
            chizel::uix! {
                #[style(BackgroundColor::hex("#ff0000"))]
                #[on(EventPin::Click(on_click_fn))]
                <ElementTestImpl name="Outer">
                    <ElementTestImpl name="Inner" />
                </ElementTestImpl>
            }
        }
    )?;
    
    // #[cfg(feature = "inspect")]
    tracing::info!("\n{:#?}", scaffold);
    Ok(ExitCode::SUCCESS)
}
