use super::{node::Node, util::handle_world_wrap_for_coords};
use crate::{
    entities::tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::{AlgorithmInUse, PathfindingNode},
};
use rand::{seq::SliceRandom, thread_rng};
use std::{collections::BinaryHeap, isize};

pub fn setup_and_run_astar(
    grid: &[&Tile],
    current_tile_id: usize,
    is_aggressive: bool,
    algo: &AlgorithmInUse,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    let mut end_tile_pos: Option<(usize, usize)> = None;
    let mut current_tile_pos: (usize, usize) = (0, 0);

    // This looks weird, probably want it dynamic some day.
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

    return astar(nodes, current_tile_pos, end_tile_pos, is_aggressive, algo);
}

fn astar(
    mut grid: Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
    is_aggressive: bool,
    algo: &AlgorithmInUse,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    let mut heap = BinaryHeap::new();
    let mut visited_order = vec![];
    let mut path = vec![];
    let end_pos = end_tile_pos.unwrap_or((0, 0));
    let h_score = hscore(
        current_tile_pos,
        end_pos,
        algo.world_wrap_enabled,
        is_aggressive,
    );
    heap.push(Node {
        distance: h_score,
        g_score: 0,
        ..grid[current_tile_pos.0][current_tile_pos.1]
    });

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

    while let Some(node) = heap.pop() {
        let current_node = &mut grid[node.row][node.col];
        if (node.row, node.col) == end_pos {
            break;
        }

        if node.visited == true || node.is_wall {
            continue;
        }

        current_node.visited = true;
        visited_order.push(PathfindingNode {
            tile_id: node.tile_id,
        });

        for (dr, dc) in directions {
            let (visit_row, visit_col) =
                handle_world_wrap_for_coords(algo, (node.row, node.col), (dr, dc))
                    .unwrap_or((usize::MAX, usize::MAX));

            if (visit_row, visit_col) == (usize::MAX, usize::MAX) {
                continue;
            }

            let checked_node = &mut grid[visit_row][visit_col];

            if checked_node.is_wall {
                continue;
            }

            let directional_distance = if dr.abs() + dc.abs() == 2 { 14 } else { 10 };

            let potential_g = node.g_score + directional_distance;

            let h_score = hscore(
                (visit_row, visit_col),
                end_pos,
                algo.world_wrap_enabled,
                is_aggressive,
            );

            if potential_g < checked_node.g_score {
                checked_node.distance = potential_g + h_score;
                checked_node.g_score = potential_g;
                checked_node.previous_node = Some((node.row, node.col));
                checked_node.visited = false;
                heap.push(Node { ..*checked_node });
            }
        }
    }
    let mut head = &grid[end_pos.0][end_pos.1];
    while let Some((row, col)) = head.previous_node {
        path.push(PathfindingNode {
            tile_id: head.tile_id,
        });
        head = &grid[row][col];
        if row == current_tile_pos.0 && col == current_tile_pos.1 {
            break;
        }
    }
    return (visited_order, path);
}

fn hscore(
    (current_row, current_col): (usize, usize),
    (end_row, end_col): (usize, usize),
    world_wrap_enabled: bool,
    is_aggressive: bool,
) -> usize {
    let mut dx = end_col as isize - current_col as isize;
    let mut dy = end_row as isize - current_row as isize;
    if dx.abs() > COL_COUNT as isize / 2 && world_wrap_enabled {
        dx = COL_COUNT as isize - dx.abs();
    }
    if dy.abs() > ROW_COUNT as isize / 2 && world_wrap_enabled {
        dy = ROW_COUNT as isize - dy.abs();
    }
    let distance_between_checked_and_end = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();

    if is_aggressive {
        (distance_between_checked_and_end as usize).pow(10)
    } else {
        distance_between_checked_and_end as usize * 10
    }
}
