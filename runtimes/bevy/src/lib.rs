// TODO: Remove this ..
#![allow(unused)]

// Required for Slate's custom allocation feature.
// TODO: Remove this when v2 is stable ..
#![feature(allocator_api)]

extern crate bevy;

// extern crate bevy_mod_picking;
// pub extern crate bevy_eventlistener as events;

//--
mod plugin;
pub use plugin::*;

pub mod config;

pub mod provider;

#[cfg(feature = "terminal")]
pub mod terminal;

pub mod window;

pub mod webview;

pub mod input;

pub mod reticle;

pub mod time;

pub mod log;
