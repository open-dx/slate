use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::PreUpdate;
use bevy::app::PluginGroup;
use bevy::app::PluginGroupBuilder;

use crate::provider::setup_new_surface;

pub struct BevySlatePlugin;

impl Plugin for BevySlatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, setup_new_surface);
    }
}

//---
/// TODO: Make configurable.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin)
            .add(bevy::core::FrameCountPlugin)
            .add(bevy::time::TimePlugin)
            .add(bevy::transform::TransformPlugin)
            .add(bevy::hierarchy::HierarchyPlugin)
            .add(bevy::diagnostic::DiagnosticsPlugin)
            .add(bevy::input::InputPlugin)
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::a11y::AccessibilityPlugin);

        #[cfg(feature = "bevy/asset")]
        {
            group = group.add(bevy::asset::AssetPlugin::default());
        }

        #[cfg(feature = "bevy/scene")]
        {
            group = group.add(bevy::scene::ScenePlugin);
        }

        #[cfg(feature = "bevy/winit")]
        {
            group = group.add(bevy::winit::WinitPlugin::default());
        }

        #[cfg(feature = "bevy/render")]
        {
            group = group
                .add(bevy::render::RenderPlugin::default())
                // NOTE: Load this after renderer initialization so that it knows about the supported
                // compressed texture formats
                .add(bevy::render::texture::ImagePlugin::default());

            #[cfg(all(not(target_arch = "wasm32"), feature = "multi-threaded"))]
            {
                group = group.add(bevy::render::pipelined_rendering::PipelinedRenderingPlugin);
            }
        }

        #[cfg(feature = "bevy/core_pipeline")]
        {
            group = group.add(bevy::core_pipeline::CorePipelinePlugin);
        }

        #[cfg(feature = "bevy/sprite")]
        {
            group = group.add(bevy::sprite::SpritePlugin);
        }

        #[cfg(feature = "bevy/text")]
        {
            group = group.add(bevy::text::TextPlugin);
        }

        #[cfg(feature = "bevy/ui")]
        {
            group = group.add(bevy::ui::UiPlugin);
        }

        #[cfg(feature = "bevy/pbr")]
        {
            group = group.add(bevy::pbr::PbrPlugin::default());
        }

        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
        #[cfg(feature = "bevy/gltf")]
        {
            group = group.add(bevy::gltf::GltfPlugin::default());
        }

        #[cfg(feature = "bevy/audio")]
        {
            group = group.add(bevy::audio::AudioPlugin::default());
        }

        #[cfg(feature = "bevy/gilrs")]
        {
            group = group.add(bevy::gilrs::GilrsPlugin);
        }

        #[cfg(feature = "bevy/animation")]
        {
            group = group.add(bevy::animation::AnimationPlugin);
        }

        #[cfg(feature = "bevy/gizmos")]
        {
            group = group.add(bevy::gizmos::GizmoPlugin);
        }

        group
    }
}
