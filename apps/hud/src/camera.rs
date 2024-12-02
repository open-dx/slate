use bevy::{
    prelude::*,
};

#[derive(Component, Debug)]
pub struct Zoom {
    pub distance: f32,
    pub min: f32,
    pub max: f32,
}

impl Zoom {
    pub fn new(distance: f32) -> Self {
        Zoom {
            distance,
            ..default()
        }
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom {
            distance: 0.5,
            min: 0.1,
            max: 5.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct Focus {
    pub center: Vec3,
}

impl Focus {
    pub fn new(center: Vec3) -> Self {
        Focus {
            center,
        }
    }
}

impl Default for Focus {
    fn default() -> Self {
        Focus {
            center: Vec3::default(),
        }
    }
}

pub fn debug_position(
    camera_query: Query<(
        &Zoom,
        &Focus,
        &Transform,
    ), (
        Changed<Transform>,
    )>,
) {
    for (_, _, transform) in camera_query.iter() {
        debug!("Camera Position: {:?}", transform);
    }
}
