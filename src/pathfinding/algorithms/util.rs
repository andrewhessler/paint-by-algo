use crate::entities::tile::{COL_COUNT, ROW_COUNT};

pub fn in_bounds(row: isize, col: isize) -> bool {
    row >= 0 && row < ROW_COUNT as isize && col >= 0 && col < COL_COUNT as isize
}
