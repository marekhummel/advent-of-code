use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let area = input.grid();
    trees(&area, 3, 1).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let area = input.grid();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .into_iter()
        .map(|(dx, dy)| trees(&area, dx, dy))
        .product::<usize>()
        .into()
}

fn trees(area: &Grid<char>, dx: usize, dy: usize) -> usize {
    (0..area.size.height)
        .step_by(dy)
        .map(|j| Index::new((j * dx / dy) % area.size.width, j))
        .filter(|idx| *area.get(idx) == '#')
        .count()
}
