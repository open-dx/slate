use std::process::ExitCode;

use anyhow::Result;

use slate::scaffold::Scaffold;
use slate::element::tests::ElementTestImpl;
use slate::event::ClickEvent;

//--
fn main() -> Result<ExitCode> {
    let interactive_ui = Scaffold::try_from_draw_fn({
        let on_click_fn = |_: &ClickEvent| {
            println!("Clicked!");
        };
        
        chizel::uix! {
            #[style(BackgroundColor::hex("#ff0000"))]
            #[on(Click, on_click_fn)]
            <ElementTestImpl name="Outer">
                <ElementTestImpl name="Inner" />
            </ElementTestImpl>
        }
    })?;
    
    // #[cfg(feature = "inspect")]
    println!("{:#?}", interactive_ui);
    
    println!("Hint: `cargo expand [-p slate] --example slate-bones` ..");
    Ok(ExitCode::SUCCESS)
}
