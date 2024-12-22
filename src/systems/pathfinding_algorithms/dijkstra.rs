use std::{cmp::Ordering, collections::BinaryHeap};

use crate::entities::tile::{Tile, COL_COUNT, ROW_COUNT};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    tile_id: usize,
    row: usize,
    col: usize,
    distance: usize,
    visited: bool,
    previous_node: Option<(usize, usize)>,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            tile_id: 0,
            row: 0,
            col: 0,
            distance: usize::MAX,
            visited: false,
            previous_node: None,
        }
    }
}

impl Node {
    fn from_tile(&mut self, tile: &Tile) {
        self.tile_id = tile.id;
        self.row = tile.row;
        self.col = tile.col;
    }
}

pub struct DijkstraOutput {
    pub tile_id: usize,
    pub event_type: DijkstraEventType,
}

pub enum DijkstraEventType {
    Visited,
    Checked,
}

pub fn setup_and_run_dijkstra(grid: &[&Tile], current_tile_id: usize) -> Vec<DijkstraOutput> {
    let mut end_tile_pos: Option<(usize, usize)> = None;
    let mut current_tile_pos: (usize, usize) = (0, 0);

    // This looks weird, probably want it dynamic some day.
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node::default(); COL_COUNT]; ROW_COUNT];

    for tile in grid {
        if tile.is_end {
            end_tile_pos = Some((tile.row, tile.col));
        }

        if tile.id == current_tile_id {
            current_tile_pos = (tile.row, tile.col);
        }
        let row = tile.row as usize;
        let col = tile.col as usize;
        nodes[row][col].from_tile(tile);
    }

    return dijkstra(nodes, current_tile_pos, end_tile_pos);
}

// Returns a vector of checked and visited events
fn dijkstra(
    mut nodes: Vec<Vec<Node>>,
    current_tile_pos: (usize, usize),
    end_tile_pos: Option<(usize, usize)>,
) -> Vec<DijkstraOutput> {
    let mut heap = BinaryHeap::new();
    let mut event_order = vec![];
    heap.push(Node {
        distance: 0,
        ..nodes[current_tile_pos.0][current_tile_pos.1]
    });

    let directions = [
        (-1, -1),
        (1, -1),
        (1, 1),
        (-1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    while let Some(mut node) = heap.pop() {
        println!("{:?}", node);
        if node.visited == true {
            continue;
        }

        if let Some(end_pos) = end_tile_pos {
            if (node.row, node.col) == end_pos {
                break;
            }
        }

        node.visited = true;
        event_order.push(DijkstraOutput {
            tile_id: node.tile_id,
            event_type: DijkstraEventType::Visited,
        });

        for (row_offset, col_offset) in directions {
            let visit_row = (node.row as isize + row_offset) as usize;
            let visit_col = (node.col as isize + col_offset) as usize;

            if visit_row >= ROW_COUNT || visit_col >= COL_COUNT {
                continue;
            }

            let checked_node = &mut nodes[visit_row][visit_col];
            println!("{}", node.distance);
            let new_distance = node.distance + 1;
            event_order.push(DijkstraOutput {
                tile_id: checked_node.tile_id,
                event_type: DijkstraEventType::Checked,
            });

            if new_distance < checked_node.distance {
                checked_node.distance = new_distance;
                checked_node.previous_node = Some((node.row, node.col));
                checked_node.visited = false;
                heap.push(Node { ..*checked_node });
            }
        }
    }
    return event_order;
}
