use bevy::prelude::*;

use crate::entities::{
    player::Player,
    tile::{Tile, TILE_SIZE},
};

pub struct SetCurrentTilePlugin;

impl Plugin for SetCurrentTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, set_current_tile_as_activated);
    }
}

fn set_current_tile_as_activated(
    player: Query<(&Transform, &Player)>,
    mut tiles: Query<(&Transform, &mut Tile)>,
) {
    let player_position = player.single().0.translation;
    for (xf, mut tile_props) in &mut tiles {
        let tile_position = &xf.translation;
        let is_in_x = player_position.x < tile_position.x + (TILE_SIZE / 2.)
            && player_position.x > tile_position.x - (TILE_SIZE / 2.);

        let is_in_y = player_position.y < tile_position.y + (TILE_SIZE / 2.)
            && player_position.y > tile_position.y - (TILE_SIZE / 2.);

        if is_in_x && is_in_y {
            println!("row {} col {} active", tile_props.row, tile_props.col);
            tile_props.activated = true;
        } else {
            tile_props.activated = false;
        }
    }
}
