use bevy::prelude::*;

#[derive(Bundle, Clone, Debug)]
pub struct PanelBundle {
    node: NodeBundle,
}

impl PanelBundle {
    pub fn new(color: Color, position: UiRect, width: Val, height: Val) -> Self {
        PanelBundle {
            node: NodeBundle {
                background_color: color.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: position.left,
                    right: position.right,
                    top: position.top,
                    bottom: position.bottom,
                    width,
                    height,
                    min_width: width,
                    ..default()
                },
                ..default()
            }
        }
    }
}

impl Default for PanelBundle {
    fn default() -> Self {
        PanelBundle {
            node: NodeBundle::default()
        }
    }
}
