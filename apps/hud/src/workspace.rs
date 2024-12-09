use bevy::prelude::*;

use crate::camera::Focus;
use crate::camera::Zoom;

#[derive(Bundle)]
pub struct WorkspaceCameraBundle {
    pub camera2d: Camera2d,
    // pub camera3d: Camera3d,
    pub focus: Focus,
    pub zoom: Zoom,
}

impl WorkspaceCameraBundle {
    pub fn new(position: Vec3, focus: Vec3) -> Self {
        WorkspaceCameraBundle {
            camera2d: Camera2d,
            // camera3d: Camera3d,
            focus: Focus::new(focus),
            zoom: Zoom::new(position.length()),
        }
    }
}

impl Default for WorkspaceCameraBundle {
    fn default() -> Self {
        WorkspaceCameraBundle {
            camera2d: Camera2d,
            // camera3d: Camera3d,
            focus: Focus::default(),
            zoom: Zoom::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct WorkspaceLightBundle {
    pub light: DirectionalLight,
    pub focus: Focus,
}

impl WorkspaceLightBundle {
    const DEFAULT_COLOR: Color = Color::WHITE;
    const DEFAULT_DISTANCE: f32 = 100.0;
    const DEFAULT_ILLUMINANCE: f32 = 80000.0;
    
    pub fn new(color: Color, position: Vec3, focus: Vec3) -> Self {
        WorkspaceLightBundle {
            light: DirectionalLight {
                color,
                ..default()
            },
            focus: Focus::new(focus),
            ..default()
        }
    }
}

impl Default for WorkspaceLightBundle {
    fn default() -> Self {
        WorkspaceLightBundle {
            light: DirectionalLight {
                color: WorkspaceLightBundle::DEFAULT_COLOR,
                ..default()
            },
            focus: Focus::default(),
        }
    }
}
