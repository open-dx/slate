#![allow(unused)]

// Required for Slate's custom allocation feature.
// TODO: Remove this.
#![feature(allocator_api)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate crossterm;

//--
mod terminal;
pub use terminal::*;

/// TODO
pub mod element;

/// TODO
pub mod event;

/// TODO
pub mod frame;