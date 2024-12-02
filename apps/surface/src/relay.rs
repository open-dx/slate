use bevy::prelude::*;

//---
/// TODO
pub struct PeerRelayPlugin;

impl PeerRelayPlugin {
    // TODO
    pub fn _new() -> Self {
        PeerRelayPlugin
    }
}

impl Plugin for PeerRelayPlugin {
    // TODO
    fn build(&self, app: &mut App) {
        // Insert the relay as a resource.
        app.insert_resource(Peers::new());
        
        // Start the relay.
        app.add_systems(Startup, PeerRelayPlugin::start);
    }
}

impl PeerRelayPlugin {
    // TODO
    fn start(
        // TODO
        _: Res<Peers>,
    ) {
        // TODO
    }
}

//---
/// TODO
#[derive(Resource)]
struct Peers {
    //..
}

impl Peers {
    // TODO
    pub fn new() -> Self {
        Peers {
            //..
        }
    }
}