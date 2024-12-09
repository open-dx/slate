use std::path::PathBuf;

use bevy::app::PostUpdate;
use bevy::window::WindowMode;
use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::PreUpdate;
use bevy::app::PluginGroup;
use bevy::app::PostStartup;
use bevy::app::PreStartup;
use bevy::app::Startup;
// use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetPlugin;
use bevy::color::Color;
use bevy::render::view::Msaa;
use bevy::render::camera::ClearColor;
use bevy::window::Window;
use bevy::window::WindowLevel;
use bevy::winit::WinitSettings;
use bevy::log::LogPlugin;

use crate::config::BevySlateConfig;
use crate::input::FocusedInput;
use crate::window::WindowPlugin;
use crate::window::WindowKind;

#[derive(Default)]
pub struct BevySlatePlugin {
    config: BevySlateConfig,
}

impl BevySlatePlugin {
    pub fn new(config: BevySlateConfig) -> Self {
        BevySlatePlugin {
            config,
        }
    }
}

impl Plugin for BevySlatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app());
        app.insert_resource(ClearColor(self.config.clear_color));
        
        app.insert_resource(FocusedInput::default());
        
        app.add_event::<crate::window::WindowEvent>();
        app.add_event::<bevy::input::keyboard::KeyboardInput>();
        
        if self.config.bevy_defaults {
            tracing::debug!("Enabling Bevy's default plugins ..");
            app.add_plugins(self.get_bevy_defaults());
        }
        
        #[cfg(feature = "terminal")] {
            app.add_plugins(crate::terminal::TerminalPlugin::new());
        }
        
        app.add_plugins(crate::reticle::ReticlePlugin::new());
        
        app.add_plugins(crate::webview::WebViewPlugin::default());
        
        // TODO: Move these to the window manager ..
        // app.add_systems(PostUpdate, crate::window::bootstrap_new_windows);
        // app.add_systems(PostUpdate, crate::window::pair_parent_child_windows);
        
        app.add_systems(Startup, crate::provider::setup_new_surface);
    }
}

impl BevySlatePlugin {
    /// Utility method returning a set of default bevy plugins optmized for Slate.
    fn get_bevy_defaults(&self) -> bevy::app::PluginGroupBuilder {
        bevy::DefaultPlugins
            // Tell the built-in Asset Plugin where to find our files.
            .set(AssetPlugin {
                file_path: self.config.asset_dir.clone(),
                watch_for_changes_override: Some(true),
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
    fn create_window(&self, title: &str, width: f32, height: f32) -> Window {
        Window {
            title: String::from(title),
            mode: WindowMode::Windowed,
            resolution: bevy::window::WindowResolution::new(width, height),
            transparent: false,
            decorations: true,
            visible: true,
            resizable: true,
            position: bevy::window::WindowPosition::Centered(bevy::window::MonitorSelection::Current),
            composite_alpha_mode: bevy::window::CompositeAlphaMode::Auto,
            window_level: WindowLevel::Normal,
            ..Default::default()
        }
    }
}
