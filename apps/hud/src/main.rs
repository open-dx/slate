#![feature(trait_alias)]
#![allow(unused)] // TODO: Remove this ..

mod hud;
mod log;
mod user;
mod input;
mod device;
mod ui;
mod camera;
mod script;
mod workspace;
mod artist;
mod tool;
mod layer;
mod doodle;

use bevy::app::App;

use hud::Hud;

fn main() {
    slate::log::init(crate::log::DEFAULT_LOG_FILTER);
    
    let mut app = App::new()
        .add_plugins(Hud::new(5))
        .run();
}
