use bevy::prelude::*;

use crate::{entities::tile::Tile, pathfinding_algorithms::dijkstra::setup_and_run_dijkstra};

use super::emit_current_tile::CurrentTileEvent;

pub struct EmitPathfindingPlugin;

impl Plugin for EmitPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, trigger_pathfinding);
    }
}

fn trigger_pathfinding(
    tiles: Query<&Tile>,
    mut current_tile_reader: EventReader<CurrentTileEvent>,
) {
    let tiles: Vec<&Tile> = tiles.iter().collect();
    for event in current_tile_reader.read() {
        setup_and_run_dijkstra(&tiles, event.id);
    }
}

// Storing this here until pathfinding
// fn draw_path_to_end(time: Res<Time>, mut tiles: Query<(&mut Tile)>, mut count: Local<u32>) {
//     *count += 1;
//     if *count == 1 {
//         let tiles: Vec<&Tile> = tiles.iter().collect();
//         for tile in tiles {
//             println!("{:?}", tile);
//         }
//     }
// }
