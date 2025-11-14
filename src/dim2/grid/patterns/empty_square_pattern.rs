use std::sync::Arc;

use crate::dim2::grid::patterns::{empty_rectangle_pattern, new_empty_rectangle_pattern};

use super::grid_pattern::GridPattern;

pub fn empty_square_pattern(radius: usize) -> Arc<GridPattern> {
    empty_rectangle_pattern(radius, radius, radius, radius)
}

pub fn empty_new_square_pattern(radius: usize) -> GridPattern {
    new_empty_rectangle_pattern(radius, radius, radius, radius)
}
