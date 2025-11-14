use std::{cell::OnceCell, collections::BTreeMap, sync::Arc};

use crate::{
    dim2::grid::{patterns::GridPattern, Grid, Position},
    FactoryCache,
};

pub fn rectangle_pattern(offset_left: usize, offset_up: usize, offset_right: usize, offset_down: usize) -> Arc<GridPattern> {
    static mut PATTERN_CACHE: OnceCell<
        FactoryCache<(usize, usize, usize, usize), GridPattern, BTreeMap<(usize, usize, usize, usize), Arc<GridPattern>>>,
    > = OnceCell::new();

    let cache = unsafe {
        #[allow(static_mut_refs)]
        PATTERN_CACHE.get_or_init(|| FactoryCache::new(BTreeMap::new(), Box::new(|(l, u, r, d)| new_rectangle_pattern(l, u, r, d))))
    };

    cache.get((offset_left, offset_up, offset_right, offset_down))
}

pub fn new_rectangle_pattern(offset_left: usize, offset_up: usize, offset_right: usize, offset_down: usize) -> GridPattern {
    let center = Position::from((offset_left, offset_up));
    let width = offset_left + 1 + offset_right;
    let height = offset_up + 1 + offset_down;

    let mapping = vec![true; width * height];
    GridPattern {
        mapping: Grid::with_preset_values(width, height, mapping.into_boxed_slice()),
        center,
    }
}
