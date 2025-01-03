use std::collections::VecDeque;

use rand::{seq::SliceRandom, thread_rng};

use crate::{
    entities::tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::{AlgorithmInUse, PathfindingNode},
};

use super::{
    node::Node,
    util::{handle_world_wrap_for_coords, in_bounds},
};

pub fn setup_and_run_bfs(
    grid: &[&Tile],
    current_tile_id: usize,
    algo: &AlgorithmInUse,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    println!("Triggered BFS");
    let mut end_tile_pos: Option<(usize, usize)> = None;
    let mut current_tile_pos: (usize, usize) = (0, 0);
    let mut visited = vec![];

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

    let path = bfs(nodes, current_tile_pos, end_tile_pos, &mut visited, algo);

    let visited = visited
        .into_iter()
        .map(|node_id| PathfindingNode { tile_id: node_id })
        .collect();
    return (visited, path);
}

fn bfs(
    mut grid: Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
    visited: &mut Vec<usize>,
    algo: &AlgorithmInUse,
) -> Vec<PathfindingNode> {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::default();
    let current_tile_node = &mut grid[current_tile_pos.0][current_tile_pos.1];
    let current_row = current_tile_node.row;
    let current_col = current_tile_node.col;

    queue.push_front((current_row, current_col));

    let mut directions = [
        (-1, -1),
        (1, -1),
        (1, 1),
        (-1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];
    directions.rotate_left(algo.direction_offset);
    if algo.random_direction {
        let mut rng = thread_rng(); // I wonder if this is expensive...
        directions.shuffle(&mut rng);
    }

    while let Some((row, col)) = queue.pop_back() {
        if grid[row][col].visited {
            continue;
        }
        grid[row][col].visited = true;
        visited.push(grid[row][col].tile_id);

        if end_tile_pos.is_some() {
            if end_tile_pos.unwrap() == (row, col) {
                break;
            }
        }
        // let mut rng = thread_rng();
        // directions.shuffle(&mut rng);

        for (dr, dc) in directions {
            let (visit_row, visit_col) = handle_world_wrap_for_coords(algo, (row, col), (dr, dc))
                .unwrap_or((usize::MAX, usize::MAX));

            if (visit_row, visit_col) == (usize::MAX, usize::MAX) {
                continue;
            }

            if !grid[visit_row][visit_col].visited {
                queue.push_front((visit_row, visit_col));
                grid[visit_row][visit_col].previous_node = Some((row, col));
            }
        }
    }

    let mut path = vec![];

    end_tile_pos.map(|(end_row, end_col)| {
        let mut path_node = &grid[end_row][end_col];
        loop {
            path.push(PathfindingNode {
                tile_id: path_node.tile_id,
            });
            if (path_node.row, path_node.col) == (current_row, current_col) {
                break;
            }
            path_node =
                &grid[path_node.previous_node.unwrap().0][path_node.previous_node.unwrap().1];
        }
    });

    return path;
}
