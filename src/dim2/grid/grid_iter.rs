use std::mem::{self, MaybeUninit};

use super::{position::Position, rect_size::RectSize, to_grid_position};

pub struct GridIter<'a, T> {
    values: &'a [T],
    size: RectSize,
    index: i64,
}

impl<'a, T> GridIter<'a, T> {
    pub fn new(values: &'a [T], size: RectSize) -> Self {
        GridIter { values, size, index: -1 }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index as usize == self.values.len() {
            return None;
        }
        Some((to_grid_position(self.index as usize, self.size), &self.values[self.index as usize]))
    }
}

pub struct GridIterMut<'a, T> {
    values: &'a mut [T],
    size: RectSize,
    index: i64,
}

impl<'a, T> GridIterMut<'a, T> {
    pub fn new(values: &'a mut [T], size: RectSize) -> Self {
        GridIterMut { values, size, index: -1 }
    }
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (Position, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index as usize == self.values.len() {
            return None;
        }
        let val: *mut T = &mut self.values[self.index as usize];
        Some((to_grid_position(self.index as usize, self.size), unsafe { val.as_mut().unwrap() }))
    }
}

pub struct GridIntoIter<T> {
    values: Box<[T]>,
    size: RectSize,
    index: usize,
}

impl<T> GridIntoIter<T> {
    pub fn new(values: Box<[T]>, size: RectSize) -> Self {
        GridIntoIter { values, size, index: 0 }
    }
}

impl<T> Iterator for GridIntoIter<T> {
    type Item = (Position, T);

    fn next(&mut self) -> Option<(Position, T)> {
        let current_index = self.index;
        self.index += 1;

        if current_index == self.values.len() {
            return None;
        }

        Some((
            to_grid_position(current_index, self.size),
            mem::replace(&mut self.values[current_index], unsafe { MaybeUninit::<T>::uninit().assume_init() }),
        ))
    }
}
