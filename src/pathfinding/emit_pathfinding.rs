use bevy::prelude::*;

use crate::{
    entities::{
        player::input::{InputAction, PlayerInput},
        tile::{emit_current::CurrentTileEvent, Tile},
    },
    terrain::tile_modifier::TerrainEvent,
};

use super::algorithms::{astar::setup_and_run_astar, dijkstra::setup_and_run_dijkstra, Algorithm};

pub struct EmitPathfindingPlugin;

impl Plugin for EmitPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathfindingEvent>()
            .add_event::<PathEvent>()
            .add_systems(Startup, setup_algo_in_use)
            .add_systems(FixedUpdate, trigger_pathfinding_by_button);
    }
}

#[derive(Event, Clone)]
pub struct PathfindingEvent {
    pub visited: Vec<PathfindingNode>,
}

#[derive(Event, Clone)]
pub struct PathEvent {
    pub nodes: Vec<PathfindingNode>,
}

#[derive(Clone, Debug)]
pub struct PathfindingNode {
    pub tile_id: usize,
}

#[derive(Debug, Clone)]
pub enum PathfindingEventType {
    Visited,
    Path,
}

#[derive(Resource)]
pub struct AlgorithmInUse {
    name: Algorithm,
}

fn setup_algo_in_use(mut commands: Commands) {
    commands.insert_resource(AlgorithmInUse {
        name: Algorithm::Dijkstra,
    });
}

fn run_algo(
    algo: &AlgorithmInUse,
    tiles: &[&Tile],
    current_tile_id: &usize,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    match algo.name {
        Algorithm::AStar => setup_and_run_astar(&tiles, *current_tile_id, false),
        Algorithm::AgressiveStar => setup_and_run_astar(&tiles, *current_tile_id, true),
        Algorithm::Dijkstra => setup_and_run_dijkstra(&tiles, *current_tile_id),
    }
}

fn trigger_pathfinding_by_button(
    tiles: Query<&Tile>,
    mut player_input_reader: EventReader<PlayerInput>,
    mut terrain_event_reader: EventReader<TerrainEvent>,
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut pathfinding_writer: EventWriter<PathfindingEvent>,
    mut path_writer: EventWriter<PathEvent>,
    mut current_tile_id: Local<usize>,
    mut pre_calced_event_list: Local<Vec<PathfindingNode>>,
    mut pre_calced_path: Local<Vec<PathfindingNode>>,
    mut algo: ResMut<AlgorithmInUse>,
) {
    for _event in terrain_event_reader.read() {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        let (visited, path) = run_algo(&*algo, &tiles, &*current_tile_id);
        *pre_calced_event_list = visited;
        *pre_calced_path = path;
    }

    for event in current_tile_reader.read() {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        *current_tile_id = event.id;
        let (visited, path) = run_algo(&*algo, &tiles, &*current_tile_id);
        *pre_calced_event_list = visited;
        *pre_calced_path = path;
    }

    for input in player_input_reader.read() {
        let needs_recalc = set_algorithm_from_key_input(input, &mut algo);
        if needs_recalc {
            let tiles: Vec<&Tile> = tiles.iter().collect();
            let (visited, path) = run_algo(&*algo, &tiles, &*current_tile_id);
            *pre_calced_event_list = visited;
            *pre_calced_path = path;
        }
        if input.action == InputAction::Pressed && input.key == KeyCode::KeyJ {
            println!(
                "Emitting {} pathfinding events",
                pre_calced_event_list.len()
            );

            pathfinding_writer.send(PathfindingEvent {
                visited: (*pre_calced_event_list).clone(),
            });
        }

        if input.action == InputAction::Pressed && input.key == KeyCode::KeyH {
            println!("Emitting {} Path Events", pre_calced_path.len());
            path_writer.send(PathEvent {
                nodes: (*pre_calced_path).clone(),
            });
        }
    }
}

fn set_algorithm_from_key_input(event: &PlayerInput, algo: &mut ResMut<AlgorithmInUse>) -> bool {
    if event.action == InputAction::Pressed {
        if event.key == KeyCode::Digit1 {
            algo.name = Algorithm::Dijkstra;
            return true;
        }

        if event.key == KeyCode::Digit2 {
            algo.name = Algorithm::AStar;
            return true;
        }

        if event.key == KeyCode::Digit3 {
            algo.name = Algorithm::AgressiveStar;
            return true;
        }
    }
    false
}
