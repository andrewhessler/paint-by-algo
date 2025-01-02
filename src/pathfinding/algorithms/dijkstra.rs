use rand::{seq::SliceRandom, thread_rng};

use super::{node::Node, util::in_bounds};
use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    entities::tile::{Tile, TileType, COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::{AlgorithmInUse, PathfindingNode},
};

pub fn setup_and_run_dijkstra(
    tiles: &[&Tile],
    current_tile_id: usize,
    algo: &AlgorithmInUse,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    let mut end_tile_pos: Option<(usize, usize)> = None;
    let mut current_tile_pos: (usize, usize) = (0, 0);

    // This looks weird, probably want it dynamic some day.
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node::default(); COL_COUNT]; ROW_COUNT];

    for tile in tiles {
        if tile.tile_type == TileType::End {
            end_tile_pos = Some((tile.row, tile.col));
        }

        if tile.id == current_tile_id {
            current_tile_pos = (tile.row, tile.col);
        }

        let row = tile.row as usize;
        let col = tile.col as usize;
        nodes[row][col].from_tile(tile);

        if tile.tile_type == TileType::Wall {
            nodes[row][col].visited = true;
            nodes[row][col].is_wall = true;
        }
    }

    return dijkstra(nodes, current_tile_pos, end_tile_pos, algo);
}

// Emits an individual Pathfinding event per visited node
fn dijkstra(
    mut nodes: Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
    algo: &AlgorithmInUse,
) -> (Vec<PathfindingNode>, Vec<PathfindingNode>) {
    let mut heap = BinaryHeap::new();
    let mut visited_order = vec![];
    let mut path = vec![];
    heap.push(Reverse(Node {
        distance: 0,
        ..nodes[current_tile_pos.0][current_tile_pos.1]
    }));

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

    let end_pos = end_tile_pos.unwrap_or((0, 0));
    while let Some(Reverse(mut node)) = heap.pop() {
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

        for (dr, dc) in directions {
            let visit_row;
            let visit_col;
            if algo.world_wrap_enabled {
                visit_row = ((node.row + ROW_COUNT) as isize + dr) as usize % ROW_COUNT; // add row count to avoid negative index >.> <.<
                visit_col = ((node.col + COL_COUNT) as isize + dc) as usize % COL_COUNT;
            } else {
                let i_visit_row = node.row as isize + dr;
                let i_visit_col = node.col as isize + dc;
                if in_bounds(i_visit_row, i_visit_col) {
                    visit_row = i_visit_row as usize;
                    visit_col = i_visit_col as usize;
                } else {
                    continue;
                }
            }

            if nodes[visit_row][visit_col].is_wall {
                continue;
            }

            let directional_distance = if dr.abs() + dc.abs() == 2 { 14 } else { 10 };

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
                heap.push(Reverse(Node { ..*checked_node }));
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
