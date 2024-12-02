use bevy::prelude::*;

//---
pub const DEFAULT_MOUSE_Z_POS: f32 = 0.0;

//---
#[derive(Debug)]
pub struct InputPlugin {
    //..
}

impl InputPlugin {
    pub fn new() -> Self {
        InputPlugin {
            //..
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputStartEvent>();
        app.add_event::<InputUpdateEvent>();
        app.add_event::<InputEndEvent>();
    }
}

//---
#[derive(Event, Default, Debug)]
pub struct InputStartEvent {
    pub position: Vec3,
}

#[derive(Event, Default, Debug)]
pub struct InputUpdateEvent {
    pub position: Vec3,
}

impl From<Vec2> for InputUpdateEvent {
    fn from(vec: Vec2) -> Self {
        InputUpdateEvent {
            position: Vec3 {
                x: vec.x.to_owned(),
                y: vec.y.to_owned(),
                z: DEFAULT_MOUSE_Z_POS,
            }
        }
    }
}

///..
#[derive(Event, Default, Debug)]
pub struct InputEndEvent {
    pub position: Vec3,
}
