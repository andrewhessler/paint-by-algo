use bevy::prelude::*;

use crate::entities::{
    player::Player,
    tile::{Tile, TileActivated, TILE_SIZE},
};

pub struct SetCurrentTilePlugin;

impl Plugin for SetCurrentTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, emit_current_tile_as_activated);
    }
}

fn emit_current_tile_as_activated(
    player: Query<(&Transform, &Player)>,
    tiles: Query<(&Transform, &Tile)>,
    mut tile_activated_writer: EventWriter<TileActivated>,
) {
    let player_position = player.single().0.translation;
    for (&xf, tile) in &tiles {
        let tile_position = &xf.translation;
        let is_in_x = player_position.x < tile_position.x + (TILE_SIZE / 2.)
            && player_position.x > tile_position.x - (TILE_SIZE / 2.);

        let is_in_y = player_position.y < tile_position.y + (TILE_SIZE / 2.)
            && player_position.y > tile_position.y - (TILE_SIZE / 2.);

        if is_in_x && is_in_y {
            tile_activated_writer.send(TileActivated { id: tile.id });
        }
    }
}
