pub mod adjacent_pattern;
pub use adjacent_pattern::*;

pub mod empty_square_pattern;
pub use empty_square_pattern::*;

pub mod cross_pattern;
pub use cross_pattern::*;

pub mod grid_pattern;
pub use grid_pattern::*;

pub mod rectangle_pattern;
pub use rectangle_pattern::*;

pub mod empty_rectangle_pattern;
pub use empty_rectangle_pattern::*;

pub mod square_pattern;
pub use square_pattern::*;

use super::{Grid, Position};

pub trait PatternPositions {
    fn get_pattern_positions(&self, center: Position, fill_center: bool) -> Box<[Position]>;
}

pub fn get_grid_values_from_pattern<T>(grid: &Grid<T>, center: Position, fill_center: bool, pattern: &dyn PatternPositions) -> Box<[T]>
where
    T: Copy,
{
    pattern
        .get_pattern_positions(center, fill_center)
        .iter()
        .filter_map(|&pos| grid.get(pos))
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
