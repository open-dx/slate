use bevy::prelude::*;

#[derive(Bundle, Clone, Debug)]
pub struct PanelBundle {
    node: Node,
}

impl PanelBundle {
    pub fn new(color: Color, position: UiRect, width: Val, height: Val) -> Self {
        PanelBundle {
            node: Node {
                position_type: PositionType::Absolute,
                left: position.left,
                right: position.right,
                top: position.top,
                bottom: position.bottom,
                width,
                height,
                min_width: width,
                ..Default::default()
        }
        }
    }
}

impl Default for PanelBundle {
    fn default() -> Self {
        PanelBundle {
            node: Node::default()
        }
    }
}
