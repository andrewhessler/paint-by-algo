use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    entities::tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::{AlgorithmInUse, PathfindingNode},
};

use super::node::Node;

/*
 * Okay here's the plan for dfs
 * From current node, add children in directions
 * Then we have tree
 * Pass tree to dfs
 * Search with DFS
 * Gonna not do a tree because the children are known
 */

/*
 * DFS would involve following the first unvisited child
 * BFS would involve seraching all of a nodes children, then moving on to proces their children
 */

pub fn setup_and_run_dfs(
    grid: &[&Tile],
    current_tile_id: usize,
    algo: &AlgorithmInUse,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    println!("Triggered DFS");
    let mut end_tile_pos: Option<(usize, usize)> = None;
    let mut current_tile_pos: (usize, usize) = (0, 0);
    let mut visited = vec![];
    let mut path = vec![];

    let mut nodes: Vec<Vec<Node>> = vec![vec![Node::default(); COL_COUNT]; ROW_COUNT];
    for tile in grid {
        let row = tile.row as usize;
        let col = tile.col as usize;
        if tile.tile_type == TileType::End {
            end_tile_pos = Some((row, col));
        }

        if tile.id == current_tile_id {
            current_tile_pos = (row, col);
        }

        nodes[row][col].from_tile(tile);

        if tile.tile_type == TileType::Wall {
            nodes[row][col].visited = true;
            nodes[row][col].is_wall = true;
        }
    }

    dfs(
        &mut nodes,
        current_tile_pos,
        end_tile_pos,
        &mut visited,
        &mut path,
        algo,
    );

    let visited = visited
        .into_iter()
        .map(|node_id| PathfindingNode { tile_id: node_id })
        .collect();
    let path = path
        .into_iter()
        .map(|node_id| PathfindingNode { tile_id: node_id })
        .collect();
    return (visited, path);
}

fn dfs(
    grid: &mut Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
    visited: &mut Vec<usize>,
    path: &mut Vec<usize>,
    algo: &AlgorithmInUse,
) -> bool {
    let current_tile_node = &mut grid[current_tile_pos.0][current_tile_pos.1];
    let current_row = current_tile_node.row;
    let current_col = current_tile_node.col;
    let current_tile_id = current_tile_node.tile_id;

    let is_end_tile = end_tile_pos
        .map(|pos| if pos == current_tile_pos { true } else { false })
        .unwrap_or(false);

    if !current_tile_node.visited {
        current_tile_node.visited = true;
        visited.push(current_tile_id);
    } else {
        return false;
    }

    if is_end_tile {
        path.push(current_tile_id);
        return true;
    }

    let mut directions = [
        (1, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];
    let mut in_path = false;

    directions.rotate_left(algo.direction_offset);
    if algo.random_direction {
        let mut rng = thread_rng(); // I wonder if this is expensive...
        directions.shuffle(&mut rng);
    }

    for (dr, dc) in directions {
        let visit_row = ((current_row + ROW_COUNT) as isize + dr) as usize % ROW_COUNT; // add row count to avoid negative index >.> <.<
        let visit_col = ((current_col + COL_COUNT) as isize + dc) as usize % COL_COUNT;
        in_path |= dfs(
            grid,
            (visit_row, visit_col),
            end_tile_pos,
            visited,
            path,
            algo,
        );
        if in_path {
            path.push(current_tile_id);
            break;
        }
    }

    return in_path;
}
