use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::user::User;

use crate::input::InputUpdateEvent;

//---
#[derive(Component, Default, Debug)]
pub struct Location {
    pub position: Vec3,
}

#[derive(Resource, Default, Debug)]
pub struct ToolManager {
    _tools: Vec<Tool>,
}

#[derive(Component, Default, Debug)]
pub struct Tool {
    pub color: Color,
}

//---
#[derive(Bundle)]
pub struct SelectionTool {
    tool: Tool,
    shape: SelectionMarquee,
}

#[derive(Bundle, Clone, Debug)]
pub struct SelectionMarquee {
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,
}

impl From<Location> for SelectionMarquee {
    fn from(location: Location) -> Self {
        SelectionMarquee {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(
                    Val::Px(location.position.x),
                    Val::Undefined,
                    Val::Undefined,
                    Val::Px(location.position.y),
                ),
                ..default()
            },
            ..default()
        }
    }
}

impl Default for SelectionMarquee {
    fn default() -> Self {
        SelectionMarquee {
            background_color: Color::NONE.into(),
            node: Default::default(),
            style: Default::default(),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

///..
pub fn debug_tool_position(
    query: Query<
        (&User, &Tool, &Location),
        Or<(Changed<User>, Changed<Tool>, Changed<Location>)>,
    >,
) {
    for (user, tool, location) in query.iter() {
        info!(
            "User {:}({:?}) is at {}:{}",
            user.name,
            tool.color,
            location.position.x,
            location.position.y,
        );
    }
}

///..
pub fn sync_tool_position(
    mut input_updates: EventReader<InputUpdateEvent>,
    mut artists: Query<
        (&User, &Tool, &mut Location),
    >,
) {
    for update_event in input_updates.iter() {
        for (_, _, mut location) in artists.iter_mut() {
            location.position = update_event.position;
        }
    }
}

pub fn handle_drag_selection(
    mouse: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut tools: Query<
        (&Tool, &Location),
        Changed<Location>,
    >,
) {
    if mouse.just_pressed(MouseButton::Left) {
        commands.spawn(SelectionMarquee {
            ..default()
        });
    }
    
    if mouse.pressed(MouseButton::Left) {
        for (_, location) in tools.iter() {
           //..
        }
    }

    if mouse.just_released(MouseButton::Left) {
        //..
    }
}

pub fn handle_click_selection(
    mouse: Res<Input<MouseButton>>,
    mut commands: Commands,
    tools: Query<
        (&Tool, &Location),
        Changed<Location>,
    >,
) {
    if mouse.just_released(MouseButton::Left) {
        for (_, location) in tools.iter() {
            info!("Clicked at {:}", location.position)
        }
    }
}
