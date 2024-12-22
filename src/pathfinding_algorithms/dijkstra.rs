use crate::entities::tile::Tile;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    tile_id: usize,
    row: i32,
    col: i32,
    distance: usize,
    visited: bool,
    previous_node: Option<(usize, usize)>,
}

impl Node {
    fn from_tile(tile: &Tile) -> Self {
        Node {
            tile_id: tile.id,
            row: tile.row,
            col: tile.col,
            distance: usize::MAX,
            visited: false,
            previous_node: None,
        }
    }
}

pub fn setup_and_run_dijkstra(grid: &mut Vec<Vec<Tile>>, current_tile_id: usize) {
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    let end_tile_id = grid
        .iter()
        .flat_map(|row| row.iter())
        .find(|tile| tile.is_end)
        .map(|tile| tile.id);

    let mut nodes: Vec<Vec<Node>> = grid
        .iter()
        .map(|row| {
            rows += 1;
            row.iter()
                .map(|tile| {
                    if rows == 1 {
                        cols += 1;
                    }
                    Node::from_tile(tile)
                })
                .collect()
        })
        .collect();

    return dijkstra(&mut nodes, current_tile_id, end_tile_id);
}

pub fn dijkstra(nodes: &mut Vec<Vec<Node>>, current_tile_id: usize, end_tile_id: Option<usize>) {
    println!("Ran Dijkstra");
}
