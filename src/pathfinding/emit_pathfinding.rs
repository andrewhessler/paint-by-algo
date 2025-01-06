use bevy::prelude::*;

use crate::current_tile::emitter::CurrentTileEvent;
use crate::input::{InputAction, KeyboardInputEvent};
use crate::{entities::tile::Tile, terrain::tile_modifier::TerrainGenerationEvent};

use super::algorithms::{
    astar::setup_and_run_astar, bfs::setup_and_run_bfs, dfs::setup_and_run_dfs,
    dijkstra::setup_and_run_dijkstra, Algorithm,
};

pub struct EmitPathfindingPlugin;

impl Plugin for EmitPathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathfindingEvent>()
            .add_event::<PathEvent>()
            .insert_resource(AlgorithmInUse {
                name: Algorithm::Dijkstra,
                direction_offset: 0,
                random_direction: false,
                world_wrap_enabled: true,
            })
            .insert_resource(Precalc {
                visited: vec![],
                path: vec![],
                current_tile: 0,
            })
            .add_systems(
                FixedUpdate,
                (
                    precalc_on_algo_change,
                    precalc_on_current_tile,
                    precalc_on_terrain_generation,
                ),
            )
            .add_systems(
                Update,
                (emit_pathfinding_by_button, set_algorithm_from_key_input),
            );
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

#[derive(Resource)]
pub struct AlgorithmInUse {
    pub name: Algorithm,
    pub direction_offset: usize,
    pub random_direction: bool,
    pub world_wrap_enabled: bool,
}

#[derive(Resource)]
pub struct Precalc {
    visited: Vec<PathfindingNode>,
    path: Vec<PathfindingNode>,
    current_tile: usize,
}

fn run_algo(
    algo: &AlgorithmInUse,
    tiles: &[&Tile],
    current_tile_id: usize,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    match algo.name {
        Algorithm::AStar => setup_and_run_astar(&tiles, current_tile_id, false, algo),
        Algorithm::AggressiveStar => setup_and_run_astar(&tiles, current_tile_id, true, algo),
        Algorithm::BFS => setup_and_run_bfs(&tiles, current_tile_id, algo),
        Algorithm::DFS => setup_and_run_dfs(&tiles, current_tile_id, algo),
        Algorithm::Dijkstra => setup_and_run_dijkstra(&tiles, current_tile_id, algo),
    }
}
fn precalc_on_terrain_generation(
    algo: Res<AlgorithmInUse>,
    tiles: Query<&Tile>,
    mut terrain_gen_reader: EventReader<TerrainGenerationEvent>,
    mut precalc: ResMut<Precalc>,
) {
    for _event in terrain_gen_reader.read() {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        let (visited, path) = run_algo(&*algo, &tiles, precalc.current_tile);
        precalc.visited = visited;
        precalc.path = path;
    }
}

fn precalc_on_current_tile(
    algo: Res<AlgorithmInUse>,
    tiles: Query<&Tile>,
    mut current_tile_reader: EventReader<CurrentTileEvent>,
    mut precalc: ResMut<Precalc>,
) {
    for event in current_tile_reader.read() {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        precalc.current_tile = event.id;
        let (visited, path) = run_algo(&*algo, &tiles, precalc.current_tile);
        precalc.visited = visited;
        precalc.path = path;
    }
}

fn precalc_on_algo_change(
    algo: Res<AlgorithmInUse>,
    tiles: Query<&Tile>,
    mut precalc: ResMut<Precalc>,
) {
    if algo.is_changed() {
        let tiles: Vec<&Tile> = tiles.iter().collect();
        let (visited, path) = run_algo(&*algo, &tiles, precalc.current_tile);
        precalc.visited = visited;
        precalc.path = path;
    }
}

fn emit_pathfinding_by_button(
    precalc: Res<Precalc>,
    mut keyboard_input_reader: EventReader<KeyboardInputEvent>,
    mut pathfinding_writer: EventWriter<PathfindingEvent>,
    mut path_writer: EventWriter<PathEvent>,
) {
    for input in keyboard_input_reader.read() {
        if input.action == InputAction::Pressed {
            match input.key {
                KeyCode::KeyJ => {
                    pathfinding_writer.send(PathfindingEvent {
                        visited: precalc.visited.clone(),
                    });
                }
                KeyCode::KeyH => {
                    let mut nodes = precalc.path.clone();
                    nodes.reverse();
                    path_writer.send(PathEvent { nodes });
                }
                _ => {}
            }
        }
    }
}

fn set_algorithm_from_key_input(
    mut keyboard_input_reader: EventReader<KeyboardInputEvent>,
    mut algo: ResMut<AlgorithmInUse>,
) {
    for event in keyboard_input_reader.read() {
        if event.action == InputAction::Pressed {
            match event.key {
                KeyCode::Digit1 => algo.name = Algorithm::Dijkstra,
                KeyCode::Digit2 => algo.name = Algorithm::AStar,
                KeyCode::Digit3 => algo.name = Algorithm::AggressiveStar,
                KeyCode::Digit4 => algo.name = Algorithm::DFS,
                KeyCode::Digit5 => algo.name = Algorithm::BFS,
                KeyCode::KeyQ => algo.direction_offset = (algo.direction_offset + 1) % 8,
                KeyCode::KeyT => algo.direction_offset = (algo.direction_offset + 7) % 8,
                KeyCode::KeyP => algo.world_wrap_enabled = !algo.world_wrap_enabled,
                KeyCode::KeyX => algo.random_direction = !algo.random_direction,
                _ => {}
            }
        }
    }
}
