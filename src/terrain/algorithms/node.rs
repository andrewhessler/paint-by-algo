use crate::entities::tile::Tile;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub tile_id: usize,
    pub row: usize,
    pub col: usize,
    pub state: NodeState,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeState {
    Path,
    Current,
    Unvisited,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            tile_id: 0,
            row: 0,
            col: 0,
            state: NodeState::Unvisited,
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
