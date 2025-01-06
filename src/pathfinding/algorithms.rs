pub mod astar;
pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod node;
mod util;

pub enum Algorithm {
    AggressiveStar,
    AStar,
    BFS,
    DFS,
    Dijkstra,
}
