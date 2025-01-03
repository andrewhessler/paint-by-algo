use crate::{
    entities::tile::{COL_COUNT, ROW_COUNT},
    pathfinding::emit_pathfinding::AlgorithmInUse,
};

pub fn in_bounds(row: isize, col: isize) -> bool {
    row >= 0 && row < ROW_COUNT as isize && col >= 0 && col < COL_COUNT as isize
}

pub fn handle_world_wrap_for_coords(
    algo: &AlgorithmInUse, // I want this like this, I think. Probably makes more sense to just be a bool.
    (row, col): (usize, usize),
    (dr, dc): (isize, isize),
) -> Option<(usize, usize)> {
    let new_row;
    let new_col;
    if algo.world_wrap_enabled {
        new_row = ((row + ROW_COUNT) as isize + dr) as usize % ROW_COUNT; // add row count to avoid negative index >.> <.<
        new_col = ((col + COL_COUNT) as isize + dc) as usize % COL_COUNT;
    } else {
        let i_visit_row = row as isize + dr;
        let i_visit_col = col as isize + dc;
        if in_bounds(i_visit_row, i_visit_col) {
            new_row = i_visit_row as usize;
            new_col = i_visit_col as usize;
        } else {
            return None;
        }
    }
    return Some((new_row, new_col));
}
