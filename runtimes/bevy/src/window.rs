// Re-export bevy::window and bevy::winit for convenience.
pub use bevy::window::*;

//--
use bevy::app::prelude::*;

use bevy::ecs::prelude::*;

use bevy::input::ButtonInput;
use bevy::input::prelude::KeyCode;

use bevy::winit::WinitPlugin;
use bevy::winit::WinitWindows;

use bevy::color::Color;
use bevy::render::camera::ClearColor;

//---
/// TODO
#[derive(Clone)]
pub struct WindowPlugin {
    /// TODO
    window_tpl: Option<Window>,
    
    /// TODO
    close_when_requested: bool,
    
    /// TODO
    exit_condition: ExitCondition,
}

impl WindowPlugin {
    /// TODO
    pub fn new<T: Into<String>, R: Into<WindowResolution>>(kind: WindowKind, title: T, size: R) -> Self {
        // TODO: Get window clear color, theme, etc from the supplied StyleGuide.
        
        WindowPlugin {
            window_tpl: Some(Window {
                title: title.into(),
                window_theme: Some(WindowTheme::Dark),
                visible: true,
                present_mode: PresentMode::AutoVsync,
                mode: match kind {
                    WindowKind::Overlay => WindowMode::BorderlessFullscreen,
                    WindowKind::Wallpaper => WindowMode::BorderlessFullscreen,
                    _ => WindowMode::Windowed,
                },
                position: WindowPosition::Centered(MonitorSelection::Current),
                resolution: match kind {
                    WindowKind::Standard => size.into(),
                    _ => WindowResolution::default()
                },
                window_level: match kind {
                    WindowKind::Overlay => WindowLevel::AlwaysOnTop,
                    WindowKind::Wallpaper => WindowLevel::AlwaysOnBottom,
                    _ => WindowLevel::Normal,
                },
                enabled_buttons: EnabledButtons {
                    minimize: true,
                    maximize: true,
                    close: true,
                },
                decorations: match kind {
                    WindowKind::Standard => true,
                    _ => false,
                },
                transparent: match kind {
                    WindowKind::Standard => false,
                    _ => true,
                },
                resizable: match kind {
                    WindowKind::Standard => true,
                    _ => true,
                },
                focused: match kind {
                    WindowKind::Standard => false,
                    _ => true,
                },
                canvas: Some(String::from("slate-surface")),
                prevent_default_event_handling: false,
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            exit_condition: ExitCondition::DontExit,
            close_when_requested: true,
        }
    }
}

impl Plugin for WindowPlugin {
    /// TODO
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.09, 0.09, 0.09)));
        
        app.add_plugins(bevy::window::WindowPlugin {
            primary_window: self.window_tpl.to_owned(),
            exit_condition: self.exit_condition.to_owned(),
            close_when_requested: self.close_when_requested,
        });
        
        app.add_plugins(WinitPlugin::<bevy::winit::WakeUp>::default());
        
        app.add_event::<WindowEvent>();
        
        app.add_systems(bevy::app::Update, route_window_events);
        app.add_systems(bevy::app::Update, toggle_fullscreen);
        app.add_systems(bevy::app::Update, toggle_decorations);
    }
}

/// TODO
pub fn route_window_events(
    mut window_evt: EventReader<WindowEvent>,
    mut windows: Query<(Entity, &mut Window)>,
    _: NonSend<WinitWindows>,
) {
    for event in window_evt.read() {
        match event {
            WindowEvent::SetWindowLevel(level) => {
                for (entity, mut window) in windows.iter_mut() {
                    if window.focused {
                        tracing::debug!("Setting window {:} level to {:?}", entity, level);
                        window.window_level = *level;
                    }
                }
            }
            WindowEvent::SetScaleFactor(action) => {
                for (entity, mut window) in windows.iter_mut() {
                    if window.focused {
                        tracing::debug!("Setting window {:} scale factor to {:?}", entity, action);
                        match action {
                            ScaleFactorAction::Reset => {
                                window.resolution.set_scale_factor_override(None);
                            }
                            ScaleFactorAction::Adjust(difference) => {
                                let current_scale_factor = window.resolution.scale_factor_override().unwrap_or(window.scale_factor() as f32);
                                let new_scale_factor = (current_scale_factor + difference).clamp(0.5, 3.0);
                                window.resolution.set_scale_factor_override(Some(new_scale_factor));
                            }
                        }
                    }
                }    
            }
            WindowEvent::Close => {
                for (entity, mut window) in windows.iter_mut() {
                    if window.focused {
                        //..
                    }
                }    
            }
        }
    }
}

/// TODO
pub fn toggle_decorations(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F10) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.decorations = !window.decorations;
        }
    }
}

/// TODO
pub fn toggle_fullscreen(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.get_single_mut() {
            match window.mode {
                WindowMode::Windowed => {
                    window.mode = WindowMode::BorderlessFullscreen;
                }
                _ => {
                    window.mode = WindowMode::Windowed;
                }
            }
        }
    } else if input.just_pressed(KeyCode::F12) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.decorations = !window.decorations;
        }
    }
}

//---
/// TODO
#[derive(Event, Debug, Clone)]
pub enum WindowEvent {
    /// TODO
    SetWindowLevel(WindowLevel),
    SetScaleFactor(ScaleFactorAction),
    Close,
}

//---
/// TODO
#[derive(Default, Copy, Clone)]
pub enum WindowKind {
    #[default]
    Standard,
    Overlay,
    Wallpaper,
}

#[derive(Debug, Clone)]
pub enum ScaleFactorAction {
    Reset,
    Adjust(f32),
}
