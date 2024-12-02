use bevy::app::prelude::*;

use bevy::ecs::prelude::*;

use bevy::math::Vec2;
use bevy::math::Vec3;

use bevy::core_pipeline::core_2d::Camera2d;

use bevy::color::Color;

use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use bevy::math::VectorSpace;
use bevy::transform::prelude::*;

use bevy::window::Window;
use bevy::window::PrimaryWindow;
use bevy::window::CursorIcon;

use bevy::gizmos::gizmos::Gizmos;

//---
/// TODO
pub struct ReticlePlugin;

impl ReticlePlugin {
    /// TODO
    pub fn new() -> Self {
        ReticlePlugin
    }
}

impl Default for ReticlePlugin {
    /// TODO
    fn default() -> Self {
        ReticlePlugin
    }
}

impl Plugin for ReticlePlugin {
    /// TODO
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
        app.add_systems(PreUpdate, Self::display);
        app.add_systems(Update, Self::position);
        app.add_systems(Update, Self::cursor);
    }
}

impl ReticlePlugin {
    /// TODO
    fn setup(
        mut commands: Commands,
    ) {
        commands.spawn(ReticleBundle::new(
            ReticleShape::Circle,
            ReticleStyle::new(Color::WHITE, 30., Some(CursorIcon::default())),
            ReticlePosition::new(Vec2::ZERO, Vec3::ZERO),
        ));
    }
    
    /// TODO
    fn display(
        windows: Query<&Window, With<PrimaryWindow>>,
        reticles: Query<(&ReticleShape, &ReticleStyle, &Transform), With<ReticleShape>>,
        mut gizmos: Gizmos,
    ) {
        if let Ok(window) = windows.get_single() {
            if window.cursor_position() != None {
                for (shape, style, transform) in reticles.iter() {
                    let position = transform.translation.truncate();
                    
                    match shape {
                        ReticleShape::Circle => {
                            gizmos.circle_2d(position, style.size, style.color);
                        }
                        ReticleShape::Square => {
                            gizmos.rect_2d(position, 0., Vec2::ONE * style.size, style.color);
                        }
                        ReticleShape::Triangle => {
                            gizmos.circle_2d(position, style.size, style.color).resolution(3);
                        }
                    }
                }
            }
        }
    }
    
    /// TODO
    fn position(
        windows: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<&Transform, (With<Camera2d>, Without<ReticlePosition>)>,
        mut reticles: Query<(&mut ReticlePosition, &mut Transform), With<ReticlePosition>>,
    ) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let size = Vec2::new(window.width() as f32, window.height() as f32);
                let screen_pos = cursor_pos;
                let cursor_pos = cursor_pos - size / 2.0;
    
                if let Some(camera_transform) = cameras.iter().next() {
                    let inverse_camera_matrix = camera_transform.compute_matrix().inverse();
                    let space_2d_pos = inverse_camera_matrix.transform_point3(Vec3::new(cursor_pos.x, -cursor_pos.y, 0.0));
    
                    for (mut position, mut transform) in reticles.iter_mut() {
                        position.screen = screen_pos;
                        position.space_2d = space_2d_pos;
                        transform.translation = space_2d_pos;
                    }
                }
            }
        }
    }
    
    /// Sync the currently selected cursor to the appropriate windows.
    fn cursor(
        mut windows: Query<&mut Window, With<PrimaryWindow>>,
        reticles: Query<&ReticleStyle, (With<ReticleShape>, Changed<ReticleStyle>)>,
    ) {
        for style in reticles.iter() {
            if let Ok(mut window) = windows.get_single_mut() {
                // window.cursor.icon = style.cursor.unwrap_or(CursorIcon::Default);
            }
        }
    }
}


/// TODO
#[derive(Bundle, Default)]
pub struct ReticleBundle {
    /// TODO
    pub kind: ReticleKind,
    
    /// TODO
    pub shape: ReticleShape,
    
    /// TODO
    pub style: ReticleStyle,
    
    /// TODO
    pub position: ReticlePosition,
    
    /// TODO
    pub transform: Transform,
}

impl ReticleBundle {
    /// TODO
    fn new(shape: ReticleShape, style: ReticleStyle, position: ReticlePosition) -> Self {
        ReticleBundle {
            kind: ReticleKind::default(),
            shape,
            style,
            position,
            transform: Transform::from_translation(Vec3::ZERO),
        }
    }
}

/// TODO
#[derive(Component, Default, Debug, Clone)]
pub struct ReticleStyle {
    /// TODO
    color: Color,
    
    /// TODO
    size: f32,
    
    /// TODO
    cursor: Option<CursorIcon>,
}

impl ReticleStyle {
    /// TODO
    pub fn new(color: Color, size: f32, cursor: Option<CursorIcon>) -> Self {
        ReticleStyle {
            color,
            size,
            cursor,
        }
    }
}

/// TODO
#[derive(Component, Default, Debug, Clone)]
pub enum ReticleKind {
    /// TODO
    #[default]
    Gizmo,
}

/// TODO
#[derive(Component, Default, Debug, Clone)]
pub enum ReticleShape {
    /// TODO
    #[default]
    Circle,
    
    /// TODO
    Square,
    
    /// TODO
    Triangle,
}

/// Provides a simplified api for tracking the position of the user's cursor
/// in screen, 2D, and 3D space.
#[derive(Component, Default, Debug, Clone)]
pub struct ReticlePosition {
    /// The screen-space position of the reticle, corrected for relevant
    /// window placement and camera position.
    pub screen: Vec2,
    
    /// The 2D space position of the reticle.
    pub space_2d: Vec3,
}

impl ReticlePosition {
    /// Default CTOR
    pub fn new(screen: Vec2, space_2d: Vec3) -> Self {
        ReticlePosition {
            screen,
            space_2d,
        }
    }
}

impl ReticlePosition {
    /// Move the reticle to the next position.
    pub fn move_to(&mut self, next_pos: Vec2) {
        self.screen = next_pos;
    }
}