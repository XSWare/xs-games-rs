use super::position::Position;

/// logical, rectangular size in 2D space
#[derive(Clone, Copy)]
pub struct RectSize {
    pub width: usize,
    pub height: usize,
}

impl RectSize {
    pub const fn new(width: usize, height: usize) -> Self {
        RectSize { width, height }
    }

    /// amount of logical spaces available
    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&'_ self) -> SizeIter<'_> {
        SizeIter {
            size: self,
            next_coords: Position { x: 0, y: 0 },
        }
    }
}

#[derive(Clone, Copy)]
pub struct SizeIter<'a> {
    size: &'a RectSize,
    next_coords: Position,
}

impl<'a> SizeIter<'a> {
    pub fn new(size: &'a RectSize) -> Self {
        SizeIter {
            size,
            next_coords: Position { x: 0, y: 0 },
        }
    }
}

impl<'a> Iterator for SizeIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_coords.y as usize >= self.size.height {
            return None;
        }

        let current_coords = self.next_coords;

        if self.next_coords.x as usize >= self.size.width - 1 {
            self.next_coords.y += 1;
            self.next_coords.x = 0;
        } else {
            self.next_coords.x += 1;
        }

        Some(current_coords)
    }
}
