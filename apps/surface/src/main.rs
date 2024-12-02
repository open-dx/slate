#![feature(allocator_api)]

mod log;
mod surface;
mod relay;

//--
use std::process::ExitCode;

use crate::surface::Surface;
use crate::surface::SurfaceError;

//---
fn main() -> Result<ExitCode, SurfaceError> {
    slate::log::init(crate::log::DEFAULT_LOG_FILTER);
    
    let mut app = Surface::new().build()?;
    
    app.run() // <3
}
