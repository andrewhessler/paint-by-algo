use std::{cmp::Ordering, usize};

use crate::entities::tile::Tile;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub tile_id: usize,
    pub row: usize,
    pub col: usize,
    pub is_wall: bool,
    pub distance: usize,
    pub g_score: usize,
    pub visited: bool,
    pub previous_node: Option<(usize, usize)>,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // backwards makes min heap
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
            is_wall: false,
            distance: usize::MAX,
            g_score: usize::MAX,
            visited: false,
            previous_node: None,
        }
    }
}

impl Node {
    pub fn from_tile(&mut self, tile: &Tile) {
        self.tile_id = tile.id;
        self.row = tile.row;
        self.col = tile.col;
    }
}
