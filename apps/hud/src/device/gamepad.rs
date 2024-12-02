use bevy::prelude::*;

use bevy::input::gamepad::GamepadEvent;

#[derive(Resource)]
pub struct MyGamepad {
    gamepad: Gamepad
}

impl From<Gamepad> for MyGamepad {
    fn from(gamepad: Gamepad) -> Self {
        MyGamepad {
            gamepad,
        }
    }
}

use bevy::input::gamepad::GamepadConnection;

pub fn debug_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for gamepad_event in gamepad_evr.read() {
        match gamepad_event {
            GamepadEvent::Connection(connection_event) => {
                match &connection_event.connection {
                    GamepadConnection::Connected(gamepad_info) => {
                        tracing::info!("Connected gamepad: {:?}", gamepad_info);
                        if my_gamepad.is_none() {
                            commands.insert_resource(MyGamepad::from(connection_event.gamepad));
                        }
                    }
                    GamepadConnection::Disconnected => {
                        if let Some(my_controller) = my_gamepad.as_deref() {
                            if my_controller.gamepad.id == connection_event.gamepad.id {
                                commands.remove_resource::<MyGamepad>();
                            }
                        }
                    }
                }
            }
            _ => {
                debug!("Unknown Gamepad Event: {:?}", gamepad_event);
            }
        }
    }
}

pub fn debug_buttons(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let my_gamepad = if let Some(my_gamepad) = my_gamepad {
        my_gamepad
    } else {
        return;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad: my_gamepad.gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad: my_gamepad.gamepad,
        axis_type: GamepadAxisType::LeftStickY,
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);

        // Example: check if the stick is pushed up
        if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
            // do something
        }
    }

    // In a real game, the buttons would be configurable, but here we hardcode them
    let jump_button = GamepadButton {
        gamepad: my_gamepad.gamepad,
        button_type: GamepadButtonType::South,
    };
    let heal_button = GamepadButton {
        gamepad: my_gamepad.gamepad,
        button_type: GamepadButtonType::East,
    };

    if buttons.just_pressed(jump_button) {
        // button just pressed: make the player jump
    }

    if buttons.pressed(heal_button) {
        // button being held down: heal the player
    }
}
