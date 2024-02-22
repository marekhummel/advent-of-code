use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution03;
impl Solution03 {
    fn trees(area: &Grid<char>, dx: usize, dy: usize) -> usize {
        (0..area.size.height)
            .step_by(dy)
            .map(|j| Index::new((j * dx / dy) % area.size.width, j))
            .filter(|idx| *area.get(idx) == '#')
            .count()
    }
}

impl Solution for Solution03 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(7),
            ProblemResult::USize(187),
            ProblemResult::USize(336),
            ProblemResult::USize(4723283400),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let area = input.grid();
        Self::trees(&area, 3, 1).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let area = input.grid();
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        slopes
            .into_iter()
            .map(|(dx, dy)| Self::trees(&area, dx, dy))
            .product::<usize>()
            .to_result()
    }
}
