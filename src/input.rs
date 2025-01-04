use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KeyboardInputEvent>()
            .add_event::<MouseInputEvent>()
            .add_systems(Update, broadcast_player_input);
    }
}

#[derive(PartialEq)]
pub enum InputAction {
    Pressed,
    Released,
}

#[derive(Event)]
pub struct KeyboardInputEvent {
    pub key: KeyCode,
    pub action: InputAction,
}

#[derive(Event)]
pub struct MouseInputEvent {
    pub key: MouseButton,
    pub action: InputAction,
}

fn broadcast_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_input_writer: EventWriter<KeyboardInputEvent>,
    mut player_mouse_input_writer: EventWriter<MouseInputEvent>,
) {
    for key in mouse.get_just_pressed() {
        player_mouse_input_writer.send(MouseInputEvent {
            key: *key,
            action: InputAction::Pressed,
        });
    }

    for key in mouse.get_just_released() {
        player_mouse_input_writer.send(MouseInputEvent {
            key: *key,
            action: InputAction::Released,
        });
    }
    for key in keys.get_just_pressed() {
        player_input_writer.send(KeyboardInputEvent {
            key: *key,
            action: InputAction::Pressed,
        });
    }

    for key in keys.get_just_released() {
        player_input_writer.send(KeyboardInputEvent {
            key: *key,
            action: InputAction::Released,
        });
    }
}
