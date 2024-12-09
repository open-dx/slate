use bevy::prelude::BuildChildren;
use bevy::prelude::ButtonBundle;
use bevy::prelude::NodeBundle;
use bevy::prelude::Res;
use bevy::prelude::TextBundle;
use bevy::app::App;
use bevy::app::Startup;
use bevy::ecs::system::Commands;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::text::TextStyle;
use bevy::ui::AlignItems;
use bevy::ui::JustifyContent;
use bevy::ui::Style;
use bevy::ui::Val;
use bevy::ui::Node;
use bevy::ui::UiRect;

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
    commands.spawn(bevy::prelude::Camera2dBundle::default());
}

/// TODO
fn spawn_webviews(
    assets: Res<AssetServer>,
    mut commands: Commands
) {
    for i in 0..2 {
        tracing::trace!("Adding webview #{} ..", i);
        
        commands.spawn(if i == 0 {
            bevy::prelude::NodeBundle {
                visibility: bevy::prelude::Visibility::Visible,
                style: Style {
                    position_type: bevy::ui::PositionType::Absolute,
                    width: bevy::ui::Val::Px(800.),
                    height: bevy::ui::Val::Px(800.),
                    top: bevy::ui::Val::Px(50.),
                    left: bevy::ui::Val::Px(50.),
                    flex_direction: bevy::ui::FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            }
        } else {
            bevy::prelude::NodeBundle {
                node: Node::DEFAULT,
                visibility: bevy::prelude::Visibility::Visible,
                style: Style {
                    position_type: bevy::ui::PositionType::Absolute,
                    width: bevy::ui::Val::Px(800.),
                    height: bevy::ui::Val::Px(800.),
                    top: bevy::ui::Val::Px(50.),
                    left: bevy::ui::Val::Px(900.),
                    flex_direction: bevy::ui::FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            }
        })
            .with_children(|browser| {
                browser.spawn(TextInput)
                    .insert(ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|input| {
                        input.spawn(TextBundle::from_section(
                            "https://",
                            TextStyle {
                                font: assets.load("./assets/fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
                
                #[cfg(all(feature="verbose", feature="inspect"))]
                tracing::debug!("Adding NodeBundle: {:#?}", node_bundle);
                
                browser.spawn(NodeBundle {
                    style: Style {
                        flex_grow: 1.,
                        flex_shrink: 1.,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                    .insert(WebViewDisplay::new());
            });
    }
}
