use bevy::prelude::*;

#[derive(PartialEq)]
pub enum InputAction {
    Pressed,
    Released,
}

#[derive(Event)]
pub struct PlayerInput {
    pub key: KeyCode,
    pub action: InputAction,
}

#[derive(Event)]
pub struct PlayerMouseInput {
    pub key: MouseButton,
    pub action: InputAction,
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInput>()
            .add_event::<PlayerMouseInput>()
            .add_systems(Update, broadcast_player_input);
    }
}

fn broadcast_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_input_writer: EventWriter<PlayerInput>,
    mut player_mouse_input_writer: EventWriter<PlayerMouseInput>,
) {
    for key in mouse.get_just_pressed() {
        player_mouse_input_writer.send(PlayerMouseInput {
            key: *key,
            action: InputAction::Pressed,
        });
    }

    for key in mouse.get_just_released() {
        player_mouse_input_writer.send(PlayerMouseInput {
            key: *key,
            action: InputAction::Released,
        });
    }
    for key in keys.get_just_pressed() {
        player_input_writer.send(PlayerInput {
            key: *key,
            action: InputAction::Pressed,
        });
    }

    for key in keys.get_just_released() {
        player_input_writer.send(PlayerInput {
            key: *key,
            action: InputAction::Released,
        });
    }
}
