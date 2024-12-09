use std::env;
use std::path::PathBuf;
use std::f32::consts::*;

use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::CompositeAlphaMode;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;
// use bevy::winit::WinitWindows;
use bevy::winit::WinitSettings;

use bevy::utils::Duration;
use bevy::log::LogPlugin;
use bevy_slate::window::WindowEvent;

// use bevy_prototype_debug_lines::*;
// use bevy_prototype_debug_lines::DebugLinesPlugin;

use crate::input::InputPlugin;

use crate::device::DeviceManager;
use crate::device::mouse;
use crate::device::gamepad;

use crate::script::Script;
use crate::script::ScriptLoader;

use crate::camera;

use crate::workspace::WorkspaceCameraBundle;
use crate::workspace::WorkspaceLightBundle;

use crate::artist;

use crate::tool::ToolManager;
use crate::tool;

//---
///..
#[derive(Debug)]
pub struct Hud {
    clear_color: Color,
    sync_delay: Duration,
}

impl Hud {
    /// The underlying window color for transparent windows.
    /// TODO: Set this to Color::None when in release mode.
    /// TODO: Set this to Color::Pink (or whatever it is) in debug mode.
    // pub const WINDOW_CLEAR_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.5);
    pub const WINDOW_CLEAR_COLOR: Color = Color::NONE;
    
    ///..
    pub fn new(sync_delay: u64) -> Self {
        Hud {
            clear_color: Self::WINDOW_CLEAR_COLOR,
            sync_delay: Duration::from_secs(sync_delay),
        }
    }
}

impl Plugin for Hud {
    fn build(&self, app: &mut App) {
        app.add_plugins(self.get_bevy_defaults());
        app.add_plugins(InputPlugin::new());
        
        app.init_asset::<Script>();
        // app.init_asset_loader::<ScriptLoader>();
        
        app.add_event::<WindowEvent>();
        
        app.insert_resource(ClearColor(self.clear_color));
        app.insert_resource(WinitSettings::desktop_app());
        app.insert_resource(BuildStatus::new(self.sync_delay));
        app.insert_resource(DeviceManager::default());
        app.insert_resource(ToolManager::default());
        
        app.add_systems(Startup, setup_overlay_window);
        app.add_systems(Update, bevy_slate::window::route_window_events);
        app.add_systems(Update, bevy_slate::window::toggle_fullscreen);
        app.add_systems(Update, bevy_slate::window::toggle_decorations);
        
        app.add_systems(Update, animate_light_direction);
        app.add_systems(Update, debug_custom_asset);
        app.add_systems(Update, debug_build_status);
        app.add_systems(Update, handle_asset_drop);
        
        app.add_systems(Update, mouse::zoom_camera);
        app.add_systems(Update, mouse::orbit_camera);
        
        app.add_systems(Update, gamepad::debug_connections);
        app.add_systems(Update, gamepad::debug_buttons);
        
        app.add_systems(Update, camera::debug_position);
        
        app.add_systems(Update, artist::draw);
        
        app.add_systems(Update, tool::sync_tool_position);
        app.add_systems(Update, tool::debug_tool_position);
        app.add_systems(Update, tool::handle_click_selection);
    }
} 

impl Hud {
    /// Utility method returning a set of default bevy plugins optmized for Slate.
    fn get_bevy_defaults(&self) -> bevy::app::PluginGroupBuilder {
        bevy::DefaultPlugins
            // Tell the built-in Asset Plugin where to find our files.
            .set(AssetPlugin {
                // file_path: String::from("./asset"),
                watch_for_changes_override: Some(false),
                ..Default::default()
            })
            // TODO: Diable the built-in WindowPlugin and use our wrapper plugin instead.
            // Ex: `.disable::<bevy::log::WindowPlugin>()`
            .set(bevy::window::WindowPlugin {
                primary_window: Some(self.create_window("HUD", 800.0, 600.0)),
                exit_condition: bevy::window::ExitCondition::OnAllClosed,
                ..Default::default()
            })
            // Prefer Slate's logging facilities.
            .disable::<bevy::log::LogPlugin>()
    }
    
    /// Utility method to create a new window descriptor for windows managed by
    /// this window factory.
    pub fn create_window(&self, title: &str, width: f32, height: f32) -> Window {
        Window {
            title: String::from(title),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            resolution: WindowResolution::new(width, height),
            position: WindowPosition::Centered(MonitorSelection::Current),
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            present_mode: PresentMode::AutoVsync,
            transparent: true,
            decorations: true,
            visible: true,
            resizable: true,
            ..Default::default()
        }
    }
}

/// Adds a Slate surface to the primary window (the overlay window).
pub fn setup_overlay_window(
    mut status: ResMut<BuildStatus>,
    assets: Res<AssetServer>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    // winit_windows: NonSend<WinitWindows>,
    // mut create_window_events: EventWriter<CreateWindow>,
    mut commands: Commands,
) {
    status.script = assets.load("scripts/test.ethos");
    
    let camera_pos = Vec3::new(0.4, 0.0, 0.0);
    let key_pos = Vec3::new(50.0, 10.0, 5.0);
    let focus_pos = Vec3::new(0.0, 0.0, 0.0);
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
    
    commands.spawn(WorkspaceCameraBundle::new(camera_pos, focus_pos));
    commands.spawn(WorkspaceLightBundle::new(Color::WHITE, key_pos, focus_pos));
    
    let mut primary_window = primary_window.single_mut();
    // primary_window.mode = WindowMode::BorderlessFullscreen;
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_secs() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

///..
pub fn handle_asset_drop(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("D&D Event: {:?}", event);
    }
}

//---
///..
#[derive(Resource, Default, Debug)]
pub struct BuildStatus {
    pub loaded: bool,
    pub complete: bool,
    pub script: Handle<Script>,
    timer: Timer,
}

impl BuildStatus {
    fn new(sync_delay: Duration) -> Self {
        BuildStatus {
            timer: Timer::new(sync_delay, TimerMode::Repeating),
            ..BuildStatus::default()
        }
    }
}

fn debug_custom_asset(
    mut status: ResMut<BuildStatus>,
    custom_assets: ResMut<Assets<Script>>,
) {
    if let Some(asset) = custom_assets.get(&status.script) {
        debug!("Custom asset loaded: {:?}", asset);
        status.complete = true;
    }
}

/// Sync the Build Status from the ethos core according to a timer.
fn debug_build_status(
    mut status: ResMut<BuildStatus>,
    time: Res<Time>,
) {
    if status.timer.tick(time.delta()).just_finished() {
        //..
    }
}

//---
///
#[derive(Component, Default, Debug)]
pub struct Health {
    pub hp: f32,
    pub extra: f32,
}

//---
#[cfg(test)]
mod tests {
    // use super::*;
}
