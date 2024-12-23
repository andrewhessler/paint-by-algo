use bevy::prelude::*;

use crate::entities::tile::Tile;

use super::{
    emit_current_tile::CurrentTileEvent, pathfinding_algorithms::dijkstra::setup_and_run_dijkstra,
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
        println!("Emitting {} pathfinding events", pathfinding_events.len());
        for event in pathfinding_events {
            pathfinding_writer.send(event);
        }
    }
}
