use bevy::prelude::*;

use crate::current_tile::emitter::CurrentTileEvent;
use crate::entities::tile::Tile;
use crate::input::{InputAction, KeyboardInputEvent};

pub struct CollidablePlugin;

impl Plugin for CollidablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedEvent>()
            .insert_resource(CollideStatus::Disabled)
            .add_systems(
                FixedUpdate,
                (emit_collided_event, set_collide_status_on_keyboard_input),
            );
    }
}

#[derive(Event)]
pub struct CollidedEvent;

#[derive(Resource, PartialEq)]
pub enum CollideStatus {
    Enabled,
    Disabled,
}

#[derive(Component)]
pub struct Collidable;

fn emit_collided_event(
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut collided_event_writer: EventWriter<CollidedEvent>,
    collide_status: Res<CollideStatus>,
    tiles: Query<&Tile, With<Collidable>>,
) {
    for event in current_tile_reader.read() {
        if *collide_status == CollideStatus::Enabled {
            for tile in &tiles {
                if tile.id == event.id {
                    collided_event_writer.send(CollidedEvent);
                }
            }
        }
    }
}

fn set_collide_status_on_keyboard_input(
    mut keyboard_input_reader: EventReader<KeyboardInputEvent>,
    mut collide_status: ResMut<CollideStatus>,
) {
    for event in keyboard_input_reader.read() {
        if event.action == InputAction::Pressed && event.key == KeyCode::KeyC {
            *collide_status = match *collide_status {
                CollideStatus::Enabled => CollideStatus::Disabled,
                CollideStatus::Disabled => CollideStatus::Enabled,
            };
        }
    }
}
