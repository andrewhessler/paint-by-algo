use bevy::prelude::*;

use crate::entities::{
    player::Player,
    tile::{Tile, TILE_SIZE},
};

#[derive(Event)]
pub(crate) struct CurrentTileEvent {
    pub id: usize,
}

pub struct EmitCurrentTilePlugin;

impl Plugin for EmitCurrentTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CurrentTileEvent>()
            .add_systems(FixedUpdate, emit_current_tile);
    }
}

fn emit_current_tile(
    player: Query<(&Transform, &Player)>,
    tiles: Query<(&Transform, &Tile)>,
    mut current_tile_writer: EventWriter<CurrentTileEvent>,
    mut prev_current_id: Local<Option<usize>>,
) {
    let player_position = player.single().0.translation;
    for (&xf, tile) in &tiles {
        let tile_position = &xf.translation;
        let is_in_x = player_position.x < tile_position.x + (TILE_SIZE / 2.)
            && player_position.x > tile_position.x - (TILE_SIZE / 2.);

        let is_in_y = player_position.y < tile_position.y + (TILE_SIZE / 2.)
            && player_position.y > tile_position.y - (TILE_SIZE / 2.);

        if is_in_x && is_in_y {
            if prev_current_id.map_or(true, |id| id != tile.id) {
                *prev_current_id = Some(tile.id);
                current_tile_writer.send(CurrentTileEvent { id: tile.id });
            }
        }
    }
}
