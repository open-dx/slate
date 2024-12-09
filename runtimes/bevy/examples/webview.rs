use bevy::prelude::BuildChildren;
use bevy::prelude::ChildBuild;
use bevy::prelude::Res;
use bevy::app::App;
use bevy::app::Startup;
use bevy::ecs::system::Commands;
use bevy::asset::AssetServer;
use bevy::ui::Node;

use bevy_slate::BevySlatePlugin;
use bevy_slate::input::TextInput;
use bevy_slate::webview::WebViewDisplay;

pub const LOG_FILTER: &str = "error,webview=trace,bevy_slate_basic=debug,bevy_slate=debug,slate=debug";

//--
fn main() {
    slate::log::init(LOG_FILTER);

    App::new()
        .add_plugins(BevySlatePlugin::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_webviews)
        .run();
}

/// TODO
pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(bevy::prelude::Camera2d);
}

/// TODO
fn spawn_webviews(
    _assets: Res<AssetServer>,
    mut commands: Commands
) {
    for i in 0..2 {
        tracing::trace!("Adding webview #{} ..", i);
        
        commands.spawn(if i == 0 {
            bevy::prelude::Node {
                position_type: bevy::ui::PositionType::Absolute,
                width: bevy::ui::Val::Px(800.),
                height: bevy::ui::Val::Px(800.),
                top: bevy::ui::Val::Px(50.),
                left: bevy::ui::Val::Px(50.),
                flex_direction: bevy::ui::FlexDirection::Column,
                ..Default::default()
        }
        } else {
            bevy::prelude::Node {
                position_type: bevy::ui::PositionType::Absolute,
                width: bevy::ui::Val::Px(800.),
                height: bevy::ui::Val::Px(800.),
                top: bevy::ui::Val::Px(50.),
                left: bevy::ui::Val::Px(900.),
                flex_direction: bevy::ui::FlexDirection::Column,
                ..Default::default()
            }
        })
            .with_children(|browser| {
                browser.spawn(TextInput)
                    .insert(bevy::prelude::Button)
                    .with_children(|input| {
                        input.spawn(bevy::prelude::Text(String::from("https://")));
                    });
                
                #[cfg(all(feature="verbose", feature="inspect"))]
                tracing::debug!("Adding NodeBundle: {:#?}", node_bundle);
                
                browser.spawn(Node {
                    flex_grow: 1.,
                    flex_shrink: 1.,
                    ..Default::default()
                })
                    .insert(WebViewDisplay::new());
            });
    }
}
