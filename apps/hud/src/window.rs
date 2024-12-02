use bevy::ecs::system::Resource;
use bevy::window::WindowDescriptor;
use bevy::window::WindowMode;
use bevy::window::WindowPosition;

pub const DEFAULT_WIDTH: f32 = 800.0;
pub const DEFAULT_HEIGHT: f32 = 640.0;

//---
///..
#[derive(Resource, Debug, Clone)]
pub struct WindowManager<'Factory> {
    window_factory: &'Factory WindowFactory,
}

impl<'Factory> WindowManager<'Factory> {
    pub fn new(window_factory: &'Factory WindowFactory) -> Self {
        WindowManager {
            window_factory,
        }
    }
}

//---
///..
#[derive(Debug, Clone)]
pub struct WindowFactory {
    base_window_descriptor: WindowDescriptor,
}

impl WindowFactory {
    pub fn new(title: &str, width: f32, height: f32, transparent: bool, decorations: bool, always_on_top: bool) -> Self {
        WindowFactory {
            base_window_descriptor: WindowDescriptor {
                title: String::from(title),
                width, height, transparent, decorations, always_on_top,
                position: WindowPosition::Centered,
                ..WindowDescriptor::default()
            },
        }
    }
}

impl Default for WindowFactory {
    fn default() -> Self {
        WindowFactory {
            base_window_descriptor: WindowDescriptor {
                title: String::from("Slate HUD"),
                mode: WindowMode::Windowed,
                position: WindowPosition::Centered,
                width: 800.0,
                height: 640.0,
                transparent: true,
                decorations: false,
                always_on_top: false,
                hittest: false,
                ..Default::default()
            },
        }
    }
}

impl WindowFactory {
    /// Utility method to create a new window descriptor for windows managed by
    /// this window factory.
    pub fn create_window_descriptor(&self) -> WindowDescriptor {
        self.base_window_descriptor.clone()
    }
}
