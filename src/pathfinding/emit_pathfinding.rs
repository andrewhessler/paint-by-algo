use bevy::prelude::*;

use crate::entities::{
    player::input::{InputAction, PlayerInput},
    tile::{emit_current::CurrentTileEvent, Tile},
};

use super::algorithms::{astar::setup_and_run_astar, dijkstra::setup_and_run_dijkstra};

pub struct EmitPathfindingPlugin;

impl Plugin for EmitPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathfindingEvent>()
            .add_systems(FixedUpdate, trigger_pathfinding_by_button);
    }
}

#[derive(Event, Clone)]
pub struct PathfindingEvent {
    pub visited: Vec<PathfindingNode>,
}

#[derive(Clone)]
pub struct PathfindingNode {
    pub tile_id: usize,
    pub event_type: PathfindingEventType,
}

#[derive(Debug, Clone)]
pub enum PathfindingEventType {
    Visited,
    Checked,
}

fn trigger_pathfinding_by_button(
    tiles: Query<&Tile>,
    mut player_input_reader: EventReader<PlayerInput>,
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut pathfinding_writer: EventWriter<PathfindingEvent>,
    mut current_tile_id: Local<usize>,
    mut pre_calced_event_list: Local<Vec<PathfindingNode>>,
) {
    for event in current_tile_reader.read() {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        *current_tile_id = event.id;
        *pre_calced_event_list = setup_and_run_astar(&tiles, *current_tile_id);
    }
    for input in player_input_reader.read() {
        if input.action == InputAction::Pressed && input.key == KeyCode::KeyJ {
            println!(
                "Emitting {} pathfinding events",
                pre_calced_event_list.len()
            );
            pathfinding_writer.send(PathfindingEvent {
                visited: (*pre_calced_event_list).clone(),
            });
        }
    }
}
