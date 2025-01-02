use bevy::prelude::*;

use crate::entities::{
    player::{
        input::{InputAction, PlayerInput},
        Player,
    },
    tile::{emit_current::CurrentTileEvent, Tile},
};

#[derive(Event)]
pub struct CollidedEvent {
    pub rebound_direction: Vec2,
}

#[derive(Resource, PartialEq)]
pub enum CollideStatus {
    Enabled,
    Disabled,
}

pub struct CollidablePlugin;

impl Plugin for CollidablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedEvent>()
            .insert_resource(CollideStatus::Disabled)
            .add_systems(
                FixedUpdate,
                (emit_collided_event, set_collide_status_on_player_input),
            );
    }
}

#[derive(Component)]
pub struct Collidable;

fn emit_collided_event(
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut collided_event_writer: EventWriter<CollidedEvent>,
    collide_status: Res<CollideStatus>,
    tiles: Query<(&Tile, &Transform), With<Collidable>>,
    player: Single<&Transform, With<Player>>,
) {
    for event in current_tile_reader.read() {
        if *collide_status == CollideStatus::Enabled {
            for (tile, xf) in &tiles {
                if tile.id == event.id {
                    let p_xf = *player;
                    let (tile_x, tile_y) = (xf.translation.x, xf.translation.y);
                    let (player_x, player_y) = (p_xf.translation.x, p_xf.translation.y);

                    let rebound_direction =
                        Vec2::new(player_x - tile_x, player_y - tile_y).normalize();

                    collided_event_writer.send(CollidedEvent { rebound_direction });
                }
            }
        }
    }
}

fn set_collide_status_on_player_input(
    mut player_input_reader: EventReader<PlayerInput>,
    mut collide_status: ResMut<CollideStatus>,
) {
    for event in player_input_reader.read() {
        if event.action == InputAction::Pressed && event.key == KeyCode::KeyC {
            *collide_status = match *collide_status {
                CollideStatus::Enabled => CollideStatus::Disabled,
                CollideStatus::Disabled => CollideStatus::Enabled,
            };
        }
    }
}
