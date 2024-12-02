use bevy::prelude::*;

///--
///...
#[derive(Component, Default)]
pub struct Doodle<TShape>
where
    TShape: DoodleShape,
{
    /// The Sprite representation of this doodle.
    pub sprite: TShape,
    /// The relative size of the doodle.
    pub size: f32,
}

///--
///..
pub trait DoodleShape
where
    Self: Send + Sync + 'static,
{
    //..
}
