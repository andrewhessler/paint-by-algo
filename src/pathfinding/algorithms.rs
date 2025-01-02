pub mod astar;
pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod node;
mod util;

pub enum Algorithm {
    AgressiveStar,
    AStar,
    BFS,
    DFS,
    Dijkstra,
}
