use bevy::prelude::*;

use crate::entities::tile::Tile;

use super::{
    emit_current_tile::CurrentTileEvent,
    pathfinding_algorithms::dijkstra::{setup_and_run_dijkstra, DijkstraEventType},
};

pub struct EmitPathfindingPlugin;

impl Plugin for EmitPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathfindingEvent>()
            .add_systems(FixedUpdate, trigger_pathfinding);
    }
}

#[derive(Event, Clone)]
pub struct PathfindingEvent {
    pub tile_id: usize,
    pub event_type: PathfindingEventType,
}

#[derive(Debug, Clone)]
pub enum PathfindingEventType {
    Visited,
    Checked,
}

fn trigger_pathfinding(
    tiles: Query<&Tile>,
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut pathfinding_writer: EventWriter<PathfindingEvent>,
) {
    let tiles: Vec<&Tile> = tiles.iter().collect();
    for current_tile_event in current_tile_reader.read() {
        let pathfinding_events = setup_and_run_dijkstra(&tiles, current_tile_event.id);
        for event in pathfinding_events {
            let event_type = match event.event_type {
                DijkstraEventType::Visited => PathfindingEventType::Visited,
                DijkstraEventType::Checked => PathfindingEventType::Checked,
            };
            pathfinding_writer.send(PathfindingEvent {
                tile_id: event.tile_id,
                event_type,
            });
        }
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
