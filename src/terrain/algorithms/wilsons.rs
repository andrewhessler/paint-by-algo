use crate::{
    entities::{
        player::input::InputAction,
        tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    },
    pathfinding::emit_pathfinding::PathfindingNode,
    terrain::tile_modifier::{BuildType, TerrainAction, TerrainEvent},
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use super::node::{Node, NodeState};

pub fn setup_and_run_wilsons(grid: &[&Tile]) -> Vec<TerrainEvent> {
    /*
     * Create a terrain event to convert every Tile to a wall
     *
     * Use a half scale of the grid converted to nodes
     * Walk a path and generate Open events for each, only 4 directions so can assume pretty
     * easily.
     *
     * I need the tile ids, so maybe I just do the whole grid with modified access.
     */
    let mut terrain_events = vec![];
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node::default(); COL_COUNT]; ROW_COUNT];
    for tile in grid {
        terrain_events.push(TerrainEvent {
            tile_id: tile.id,
            action: TerrainAction::Added,
            build_type: BuildType::Wall,
        });
        nodes[tile.row][tile.col].from_tile(tile);
    }
    wilsons(nodes, &mut terrain_events);
    terrain_events
}

pub fn wilsons(mut grid: Vec<Vec<Node>>, terrain_events: &mut Vec<TerrainEvent>) {
    let mut rng = thread_rng();

    let seed_row = rng.gen_range(0..ROW_COUNT / 2) * 2;
    let seed_col = rng.gen_range(0..COL_COUNT / 2) * 2;
    grid[seed_row][seed_col].state = NodeState::Path;
    terrain_events.push(TerrainEvent {
        tile_id: grid[seed_row][seed_col].tile_id,
        action: TerrainAction::Removed,
        build_type: BuildType::Wall,
    });

    while let Some((row, col)) = pick_random_unvisited(&grid, &mut rng) {
        println!("random unvisited: {}, {}", row, col);
        println!("random seed: {}, {}", seed_row, seed_col);
        let path = random_walk(row, col, &mut grid, &mut rng);

        for &(r, c) in &path {
            grid[r][c].state = NodeState::Path;
            terrain_events.push(TerrainEvent {
                tile_id: grid[r][c].tile_id,
                action: TerrainAction::Removed,
                build_type: BuildType::Wall,
            });
        }
    }
}
fn pick_random_unvisited(grid: &Vec<Vec<Node>>, rng: &mut ThreadRng) -> Option<(usize, usize)> {
    let unvisited_nodes: Vec<(usize, usize)> = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|node| node.state == NodeState::Unvisited && node.row % 2 == 0 && node.col % 2 == 0)
        .map(|node| (node.row, node.col))
        .collect();

    if unvisited_nodes.is_empty() {
        None
    } else {
        Some(*unvisited_nodes.choose(rng).unwrap())
    }
}

fn random_walk(
    start_row: usize,
    start_col: usize,
    grid: &mut Vec<Vec<Node>>,
    rng: &mut ThreadRng,
) -> Vec<(usize, usize)> {
    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
    let in_bounds = |row: isize, col: isize| -> bool {
        row >= 0 && row < ROW_COUNT as isize && col >= 0 && col < COL_COUNT as isize
    };

    if grid[start_row][start_col].state != NodeState::Unvisited {
        return vec![];
    }

    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut path: Vec<(usize, usize)> = vec![(start_row, start_col)];
    loop {
        let &(dr, dc) = directions.choose(rng).unwrap();
        let new_row = current_row as isize + dr;
        let new_col = current_col as isize + dc;

        if in_bounds(new_row, new_col) {
            let u_new_row = new_row as usize;
            let u_new_col = new_col as usize;
            if grid[u_new_row][u_new_col].state == NodeState::Current {
                for (row, col) in path {
                    grid[row][col].state = NodeState::Unvisited;
                }
                path = vec![(u_new_row, u_new_col)]; // I know this is wrong, just want to see what
                grid[u_new_row][u_new_col].state = NodeState::Current; // happens
            }

            if grid[u_new_row][u_new_col].state == NodeState::Path {
                let intermediate_node =
                    &mut grid[(new_row - dr / 2) as usize][(new_col - dc / 2) as usize];
                intermediate_node.state = NodeState::Current;
                path.push((intermediate_node.row, intermediate_node.col));
                return path;
            }

            let current_node = &mut grid[current_row][current_col];
            current_node.state = NodeState::Current;
            path.push((current_row, current_col));

            let intermediate_node =
                &mut grid[(new_row - dr / 2) as usize][(new_col - dc / 2) as usize];
            intermediate_node.state = NodeState::Current;
            path.push((intermediate_node.row, intermediate_node.col));

            current_row = new_row as usize;
            current_col = new_col as usize;
        }
    }
}
