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

pub fn setup_and_run_dijkstra(grid: &[&Tile], current_tile_id: usize) {
    let mut end_tile_id: Option<usize> = None;

    // This looks weird, probably want it dynamic some day.
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node::default(); COL_COUNT]; ROW_COUNT];

    for tile in grid {
        if tile.is_end {
            end_tile_id = Some(tile.id);
        }
        let row = tile.row as usize;
        let col = tile.col as usize;
        nodes[row][col].from_tile(tile);
    }

    return dijkstra(&mut nodes, current_tile_id, end_tile_id);
}

fn dijkstra(nodes: &mut Vec<Vec<Node>>, current_tile_id: usize, end_tile_id: Option<usize>) {
    println!(
        "Ran Dijkstra {}, {}, {}",
        current_tile_id,
        end_tile_id.unwrap_or_default(),
        nodes.len()
    );
}
