use std::sync::Arc;

use crate::dim2::grid::patterns::{new_rectangle_pattern, rectangle_pattern};

use super::grid_pattern::GridPattern;

pub fn square_pattern(radius: usize) -> Arc<GridPattern> {
    rectangle_pattern(radius, radius, radius, radius)
}

pub fn new_square_pattern(radius: usize) -> GridPattern {
    new_rectangle_pattern(radius, radius, radius, radius)
}
