#![allow(unused, unreachable_code)]

use std::process::ExitCode;

use anyhow::Result;

use slate::scaffold::Scaffold;
use slate::element::tests::ElementTestImpl;

//--
fn main() -> Result<ExitCode> {
    println!("{:#?}", {
        Scaffold::try_from_draw_fn(#[cfg(feature = "bump")] todo!("BUMP"), {
            let width = 100.;
            let height = 100.;
            
            // chizel::uix! {
            //     // TODO: Style Blocks:
            //     // #[class(btn, cmd, woop)]
            //     .cmd, .woop {
            //         BackgroundColor::hex("#00ff00"),
            //         Margin::new(0.),
            //         Padding::new(0.),
            //         BoxSize::xy(width, height),
            //     }
                
            //     #[style(BoxSize::xy(100., 100.))]
            //     #[style(BorderWeight::all(1., Percent(0.), 1., None))]
            //     #[class(cmd, woop)]
            //     <ElementTestImpl
            //         name="Thanks for using Slate! 💖" />
            // }
            |scaffold| {
                Ok(())
            }
        })?
    });
    
    Ok(ExitCode::SUCCESS)
}
