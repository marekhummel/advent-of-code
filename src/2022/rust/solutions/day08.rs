use aoc_lib::cartesian::Direction;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution08;
impl Solution08 {}

impl Solution for Solution08 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(21),
            ProblemResult::U32(1672),
            ProblemResult::U32(8),
            ProblemResult::U32(327180),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let forest = input.grid().map_elements(|c| *c as u8 - b'0');

        let mut visible = 0u32;
        for (idx, height) in forest.enumerate() {
            let is_visible = Direction::compass().into_iter().any(|dir| {
                let mut tree = idx;
                while let Some(nb) = tree.advance_check(dir, forest.size) {
                    if forest.get(&nb) >= height {
                        return false;
                    }
                    tree = nb;
                }
                true
            });

            if is_visible {
                visible += 1;
            }
        }

        visible.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let forest = input.grid().map_elements(|c| *c as u8 - b'0');

        let scenic_scores = forest.enumerate().map(|(idx, height)| {
            let views = Direction::compass().into_iter().map(|dir| {
                let mut view_range = 0u32;
                let mut tree = idx;
                while let Some(nb) = tree.advance_check(dir, forest.size) {
                    view_range += 1;
                    if forest.get(&nb) >= height {
                        break;
                    }
                    tree = nb;
                }
                view_range
            });

            views.product::<u32>()
        });

        scenic_scores.max().unwrap().to_result()
    }
}
