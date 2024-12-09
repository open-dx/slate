use bevy::ecs::prelude::*;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::ChildBuild as _;

use crate::tool::SelectionLasso;
use crate::tool::Tool;
use crate::tool::Location;
// use crate::doodle::{Doodle, DoodleShape};

///..
#[derive(Component, Default)]
pub struct Artist {
    // doodles: Vec<Entity>,
}

impl Artist {
    // pub fn draw_doodle<TShape>(&mut self, sprite: TShape, size: f32) -> Doodle<TShape>
    // where
    //     TShape: DoodleShape,
    // {
    //     Doodle::<TShape> {
    //         sprite,
    //         size,
    //     }
    // }
}

#[derive(Bundle, Default)]
pub struct ArtistBundle {
    artist: Artist,
    tool: Tool,
    location: Location,
}

///..
pub fn draw(
    mut cmds: Commands,
    mut artist_qry: Query<
        (Entity, &mut Artist),
        With<Artist>,
    >,
) {
    for (entity, _) in artist_qry.iter_mut() {
        cmds.entity(entity).with_children(|builder| {
            builder.spawn(SelectionLasso::default());
        });
    }
}
