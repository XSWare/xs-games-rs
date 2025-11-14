use std::{cell::OnceCell, collections::BTreeMap, sync::Arc};

use crate::{
    dim2::grid::{Grid, Position},
    factory_cache::FactoryCache,
};

use super::grid_pattern::GridPattern;

pub fn cross_pattern(arm_length: usize) -> Arc<GridPattern> {
    static mut PATTERN_CACHE: OnceCell<FactoryCache<usize, GridPattern, BTreeMap<usize, Arc<GridPattern>>>> = OnceCell::new();
    #[allow(static_mut_refs)]
    let cache = unsafe { PATTERN_CACHE.get_or_init(|| FactoryCache::new(BTreeMap::new(), Box::new(new_cross_pattern))) };
    cache.get(arm_length)
}

pub fn new_cross_pattern(arm_length: usize) -> GridPattern {
    let center = Position::new(arm_length as i64, arm_length as i64);

    let grid_size = 2 * arm_length + 1;
    let grid_values: Vec<_> = (0..grid_size * grid_size).map(|_| false).collect();
    let mut mapping = Grid::with_preset_values(grid_size, grid_size, grid_values.into_boxed_slice());
    for (pos, matches) in mapping.iter_mut_with_position() {
        if pos.x == center.x || pos.y == center.y {
            *matches = true;
        }
    }
    GridPattern { mapping, center }
}
