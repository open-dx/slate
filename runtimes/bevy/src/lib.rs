// Required for Slate's custom allocation feature.
// TODO: Remove this.
#![feature(allocator_api)]

extern crate bevy;

//--
mod plugin;
pub use plugin::*;

pub mod provider;

pub mod log;
