use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use grid_iter::{GridIntoIter, GridIter, GridIterMut};

pub mod grid_iter;
pub mod pathing;
pub mod patterns;
pub mod position;
pub use position::Position;
pub mod rect_size;
pub use rect_size::RectSize;
pub mod screen_translation;
pub use screen_translation::*;

pub fn to_grid_index(position: Position, size: RectSize) -> usize {
    position.y as usize * size.width + position.x as usize
}

pub fn to_grid_position(index: usize, size: RectSize) -> Position {
    Position {
        x: (index % size.width) as i64,
        y: (index / size.height) as i64,
    }
}

#[derive(Clone)]
pub struct Grid<T>
where
    T: Copy,
{
    size: RectSize,
    values: Box<[T]>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(width: usize, height: usize, initializer_value: T) -> Self {
        Self::with_preset_values(width, height, vec![initializer_value; height * width].into_boxed_slice())
    }

    pub fn with_default_values(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new(width, height, T::default())
    }

    pub const fn with_preset_values(width: usize, height: usize, values: Box<[T]>) -> Self {
        assert!(values.len() == height * width);
        Self {
            size: RectSize { width, height },
            values,
        }
    }

    pub const fn size(&self) -> RectSize {
        self.size
    }

    pub const fn width(&self) -> usize {
        self.size.width
    }

    pub const fn height(&self) -> usize {
        self.size.height
    }

    pub const fn len(&self) -> usize {
        self.values.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, position: Position) -> Option<T> {
        if self.in_bounds(position) {
            Some(self.values[to_grid_index(position, self.size)])
        } else {
            None
        }
    }

    // TODO: add error type
    pub fn set(&mut self, position: Position, value: T) -> Result<(), ()> {
        if self.in_bounds(position) {
            self.values[to_grid_index(position, self.size)] = value;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        if self.in_bounds(position) {
            Some(&mut self.values[to_grid_index(position, self.size)])
        } else {
            None
        }
    }

    pub fn get_sub_grid(&self, offset: Position, size: RectSize) -> Option<Grid<T>> {
        let mut values = Vec::new();
        for pos in size.iter() {
            let grid_pos = offset + pos;
            if !self.in_bounds(offset + pos) {
                return None;
            }

            values.push(self.values[to_grid_index(grid_pos, self.size)]);
        }

        Some(Grid::with_preset_values(size.width, size.height, values.into_boxed_slice()))
    }

    pub fn into_array(self) -> Box<[T]> {
        self.values
    }

    pub fn iter_with_position(&self) -> GridIter<T> {
        GridIter::new(&self.values, self.size)
    }

    pub fn iter_mut_with_position(&mut self) -> GridIterMut<T> {
        GridIterMut::new(&mut self.values, self.size)
    }

    pub fn into_iter_with_position(self) -> GridIntoIter<T> {
        GridIntoIter::new(self.values, self.size)
    }

    pub fn iter(&self) -> Iter<T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.values.iter_mut()
    }

    const fn in_bounds(&self, position: Position) -> bool {
        position.x >= 0 && position.y >= 0 && (position.x as usize) < self.size.width && (position.y as usize) < self.size.height
    }
}

impl<T> Index<(usize, usize)> for &Grid<T>
where
    T: Copy,
{
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[to_grid_index(Position::from(index), self.size)]
    }
}

impl<T> Index<Position> for &Grid<T>
where
    T: Copy,
{
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        &self.values[to_grid_index(index, self.size)]
    }
}

impl<T> Index<Position> for Grid<T>
where
    T: Copy,
{
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        &self.values[to_grid_index(index, self.size)]
    }
}

impl<T> IndexMut<Position> for Grid<T>
where
    T: Copy,
{
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.values[to_grid_index(index, self.size)]
    }
}
