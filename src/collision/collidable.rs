use bevy::prelude::*;

use crate::entities::{
    player::Player,
    tile::{emit_current_tile::CurrentTileEvent, Tile},
};

#[derive(Event)]
pub struct CollidedEvent {
    pub rebound_direction: Vec2,
}

pub struct CollidablePlugin;

impl Plugin for CollidablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollidedEvent>()
            .add_systems(FixedUpdate, emit_collided_event);
    }
}

#[derive(Component)]
pub struct Collidable;

fn emit_collided_event(
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut collided_event_writer: EventWriter<CollidedEvent>,
    tiles: Query<(&Tile, &Transform), With<Collidable>>,
    player: Single<&Transform, With<Player>>,
) {
    for event in current_tile_reader.read() {
        for (tile, xf) in tiles.iter() {
            if tile.id == event.id {
                let p_xf = *player;
                let (tile_x, tile_y) = (xf.translation.x, xf.translation.y);
                let (player_x, player_y) = (p_xf.translation.x, p_xf.translation.y);

                let rebound_direction = Vec2::new(player_x - tile_x, player_y - tile_y).normalize();

                collided_event_writer.send(CollidedEvent { rebound_direction });
            }
        }
    }
}
