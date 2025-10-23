use xs_games_rs::dim2::grid::{to_grid_position, Grid};

#[test]
fn iterate_over_end() {
    let grid = Grid::with_preset_values(2, 2, Box::new([1; 4]));

    let mut iter = grid.iter_with_position();

    for _ in 0..4 {
        assert!(iter.next().is_some());
    }

    assert!(iter.next().is_none());
}

#[test]
fn iterate_mutable() {
    let mut grid = Grid::with_preset_values(2, 2, Box::new([1; 4]));

    for i in grid.iter_mut_with_position() {
        *i.1 = i.0.x;
        assert_eq!(*i.1, i.0.x);
    }
}

#[test]
fn iterate_move() {
    let grid = Grid::with_preset_values(2, 2, Box::new([1; 4]));
    let size = grid.size();
    let mut list: Vec<i32> = vec![];
    let mut index: i64 = -1;
    for i in grid.into_iter_with_position() {
        index += 1;
        list.push(i.1);
        assert_eq!(to_grid_position(index as usize, size), i.0);
    }
}
