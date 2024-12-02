use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;

use crate::{
    camera::Zoom,
    camera::Focus,
};
 
pub fn zoom_camera(
    mut scroll_ev: EventReader<MouseWheel>,
    mut camera_query: Query<(
        &mut Zoom,
        &mut Transform,
    )>,
) {
    let mut zoom_delta = 0.0;
    for ev in scroll_ev.read() {
        zoom_delta += ev.y;
    }

    if zoom_delta.abs() > 0.0 {
        for (mut zoom, mut transform) in camera_query.iter_mut() {
            zoom.distance += zoom.distance * zoom_delta * 0.2;
            zoom.distance = f32::max(zoom.distance, zoom.min);
            zoom.distance = f32::min(zoom.distance, zoom.max);
            
            let center = Vec3::new(0.0, 0.0, 0.0);
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = center + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, zoom.distance));
        }
    }
}

pub fn orbit_camera(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut motion_ev: EventReader<MouseMotion>,
    mut camera_query: Query<(
        &Zoom,
        &Focus,
        &mut Transform,
        &Projection,
    )>,
) {
    let mut cursor_delta = Vec2::ZERO;
    for ev in motion_ev.read() {
        if mouse.pressed(MouseButton::Right) {
            cursor_delta += ev.delta;
        }
    }
    
    let primary_window = primary_window.single();
    for (zoom, orbit, mut transform, projection) in camera_query.iter_mut() {
        if cursor_delta.length_squared() > 0.0 {
            let viewport_size = Vec2::new(primary_window.width(), primary_window.height());
            
            let delta_x = cursor_delta.x / viewport_size.x * PI * 2.0;
            let delta_y = cursor_delta.y / viewport_size.y * PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;
            
            if let Projection::Perspective(projection) = projection {
                cursor_delta *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / viewport_size;
            }
            let right = transform.rotation * Vec3::X * -cursor_delta.x;
            let up = transform.rotation * Vec3::Y * cursor_delta.y;
            transform.translation = (right + up) * zoom.distance;
            
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = orbit.center + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, zoom.distance));
        }
    }
}
