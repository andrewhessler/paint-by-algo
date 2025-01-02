use std::{collections::BinaryHeap, isize};

use crate::{
    entities::tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::{AlgorithmInUse, PathfindingNode},
};

use super::node::Node;

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

    return astar(
        nodes,
        current_tile_pos,
        end_tile_pos,
        is_aggressive,
        algo.direction_offset,
    );
}

fn astar(
    mut nodes: Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
    is_aggressive: bool,
    offset: usize,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    let mut heap = BinaryHeap::new();
    let mut visited_order = vec![];
    let mut path = vec![];
    heap.push(Node {
        distance: 0,
        ..nodes[current_tile_pos.0][current_tile_pos.1]
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
    directions.rotate_left(offset);
    let end_pos = end_tile_pos.unwrap_or((0, 0));

    while let Some(mut node) = heap.pop() {
        if node.visited == true || node.is_wall {
            continue;
        }

        if (node.row, node.col) == end_pos {
            break;
        }

        node.visited = true;
        visited_order.push(PathfindingNode {
            tile_id: node.tile_id,
        });

        for (row_offset, col_offset) in directions {
            let visit_row = ((node.row + ROW_COUNT) as isize + row_offset) as usize % ROW_COUNT; // add row count to avoid negative index >.> <.<
            let visit_col = ((node.col + COL_COUNT) as isize + col_offset) as usize % COL_COUNT;

            if nodes[visit_row][visit_col].is_wall {
                continue;
            }

            let mut directional_distance = if row_offset.abs() + col_offset.abs() == 2 {
                14
            } else {
                10
            };

            let mut dx = end_pos.1 as isize - visit_col as isize;
            let mut dy = end_pos.0 as isize - visit_row as isize;
            if dx.abs() > COL_COUNT as isize / 2 {
                dx = COL_COUNT as isize - dx.abs();
            }
            if dy.abs() > ROW_COUNT as isize / 2 {
                dy = ROW_COUNT as isize - dy.abs();
            }
            let distance_between_checked_and_end = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();

            directional_distance += if is_aggressive {
                (distance_between_checked_and_end as usize).pow(10)
            } else {
                distance_between_checked_and_end as usize
            };

            let checked_node = &mut nodes[visit_row][visit_col];
            let new_distance = node.distance + directional_distance;
            // event_order.push(PathfindingEvent {
            //     tile_id: checked_node.tile_id,
            //     event_type: PathfindingEventType::Checked,
            // });

            if new_distance < checked_node.distance {
                checked_node.distance = new_distance;
                checked_node.previous_node = Some((node.row, node.col));
                checked_node.visited = false;
                heap.push(Node { ..*checked_node });
            }
        }
    }
    let mut head = &nodes[end_pos.0][end_pos.1];
    while let Some((row, col)) = head.previous_node {
        path.push(PathfindingNode {
            tile_id: head.tile_id,
        });
        head = &nodes[row][col];
        if row == current_tile_pos.0 && col == current_tile_pos.1 {
            break;
        }
    }
    return (visited_order, path);
}
