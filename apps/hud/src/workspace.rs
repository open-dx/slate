use bevy::prelude::*;

use crate::camera::Focus;
use crate::camera::Zoom;

#[derive(Bundle)]
pub struct WorkspaceCameraBundle {
    pub camera2d: Camera2dBundle,
    // pub camera3d: Camera3dBundle,
    pub focus: Focus,
    pub zoom: Zoom,
}

impl WorkspaceCameraBundle {
    pub fn new(position: Vec3, focus: Vec3) -> Self {
        WorkspaceCameraBundle {
            camera2d: Camera2dBundle {
                camera: Camera {
                    is_active: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            // camera3d: Camera3dBundle {
            //     camera: Camera {
            //         is_active: true,
            //         priority: 2,
            //         ..default()
            //     },
            //     transform: Transform::from_translation(position).looking_at(focus, Vec3::Y),
            //     ..default()
            // },
            focus: Focus::new(focus),
            zoom: Zoom::new(position.length()),
        }
    }
}

impl Default for WorkspaceCameraBundle {
    fn default() -> Self {
        WorkspaceCameraBundle {
            camera2d: Camera2dBundle::default(),
            // camera3d: Camera3dBundle::default(),
            focus: Focus::default(),
            zoom: Zoom::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct WorkspaceLightBundle {
    pub light_bundle: DirectionalLightBundle,
    pub focus: Focus,
}

impl WorkspaceLightBundle {
    const DEFAULT_COLOR: Color = Color::WHITE;
    const DEFAULT_DISTANCE: f32 = 100.0;
    const DEFAULT_ILLUMINANCE: f32 = 80000.0;
    
    pub fn new(color: Color, position: Vec3, focus: Vec3) -> Self {
        WorkspaceLightBundle {
            light_bundle: DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color,
                    ..default()
                },
                transform: Transform::from_translation(position).looking_at(focus, Vec3::Y),
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
            light_bundle: DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: WorkspaceLightBundle::DEFAULT_COLOR,
                    illuminance: WorkspaceLightBundle::DEFAULT_ILLUMINANCE,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO + WorkspaceLightBundle::DEFAULT_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            focus: Focus::default(),
        }
    }
}
