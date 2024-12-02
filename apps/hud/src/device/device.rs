use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

use crate::input::InputUpdateEvent;

//---
/// A device is an abstraction for
#[derive(Debug)]
pub struct Device(pub(crate) u64);

///..
#[derive(Resource, Debug)]
pub struct DeviceManager {
    // devices: Map<Device, Entity>,
}

impl Default for DeviceManager {
    fn default() -> Self {
        DeviceManager {
            //..
        }
    }
}

//---
#[derive(Debug)]
pub struct Keyboard {
    
}

/// TODO
#[derive(Debug)]
pub struct KeyCombo {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub keys: Vec<KeyCode>,
}

/// TODO
#[allow(dead_code)]
pub fn debug_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut key_events: EventReader<KeyboardInput>,
) {
    for event in key_events.read() {
        debug!(
            "Keyboard Event (ctrl={}; alt={}): {:?}",
            keys.any_pressed([
                KeyCode::ControlLeft,
                KeyCode::ControlRight,
            ]),
            keys.any_pressed([
                KeyCode::AltLeft,
                KeyCode::AltRight,
            ]),
            event,
        );
    }
}

//---
/// Update the tool position for the current user.
// TODO: Only update the position for the current user.
#[allow(dead_code)]
pub fn sync_screen_position(
    mut cursor_moved: EventReader<CursorMoved>,
    mut input_updates: EventWriter<InputUpdateEvent>,
) {
    for event in cursor_moved.read() {
        input_updates.send(InputUpdateEvent::from(event.position));
    };
}
