use super::PatternPositions;
use crate::dim2::grid::{Grid, Position};

pub struct GridPattern {
    pub mapping: Grid<bool>,
    pub center: Position,
}

impl PatternPositions for GridPattern {
    fn get_pattern_positions(&self, center: Position, fill_center: bool) -> Box<[Position]> {
        let mut result = vec![];

        let offset = center - self.center;

        for (mapping_pos, &matches) in self.mapping.iter_with_position() {
            let pos = mapping_pos + offset;
            if matches && (fill_center || pos != center) {
                result.push(pos);
            }
        }

        result.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::dim2::grid::patterns::{adjacent_pattern, PatternPositions};

    #[test]
    fn empty_center() {
        let center = (3, 4).into();

        let pattern = adjacent_pattern();
        let positions = pattern.get_pattern_positions(center, false);
        assert!(!positions.contains(&center));
    }
}
