use bevy::prelude::*;

pub enum InputAction {
    Pressed,
    Released,
}

#[derive(Event)]
pub struct PlayerInput {
    pub key: KeyCode,
    pub action: InputAction,
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInput>()
            .add_systems(Update, broadcast_player_input);
    }
}

fn broadcast_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_input_writer: EventWriter<PlayerInput>,
) {
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
