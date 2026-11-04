use aoc_lib::cartesian::{Direction, Grid};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution20;
impl Solution20 {
    fn parse(input: ProblemInput) -> (Vec<bool>, Grid<bool>) {
        let lines = input.lines();

        // Either empty windows stay off, or they turn on and fully turned on windows turn off again next iteration.
        let enhancement_key = lines[0].bytes().map(|b| b == b'#').collect_vec();
        assert!(!enhancement_key[0] || !enhancement_key[511]);

        let image = Grid::new(lines[2..].iter().map(|r| r.bytes().collect()).collect());

        (enhancement_key, image.map_elements(|b| *b == b'#'))
    }

    fn enhance_image(mut initial: Grid<bool>, enhancement_key: &[bool], iterations: usize) -> Grid<bool> {
        // Extend grid
        for dir in Direction::compass() {
            for _ in 0..iterations + 1 {
                initial.extend(dir, false);
            }
        }

        // Enhance
        (0..iterations).fold(initial, |image, iteration| {
            Self::enhance_single(&image, enhancement_key, iteration)
        })
    }

    fn enhance_single(image: &Grid<bool>, enhancement_key: &[bool], iteration: usize) -> Grid<bool> {
        let mut new_image = Grid::empty(image.size, false);
        for idx in image.size.indices() {
            let idcs = idx.moore_neighbors(image.size).into_iter().chain([idx]).sorted();
            let window = idcs.map(|i| image.get(&i)).collect_vec();

            // When computing the edges, we need to define how to expand the window.
            if window.len() < 9 {
                // If the empty space is toggling, then the edges will be on in every odd iteration
                if enhancement_key[0] && iteration & 1 == 0 {
                    new_image.set(&idx, true);
                }
                continue;
            }

            // Normal window
            let num = window.into_iter().fold(0, |n, b| (n << 1) + (*b as usize));
            new_image.set(&idx, enhancement_key[num]);
        }
        new_image
    }
}

impl Solution for Solution20 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(35),
            ProblemResult::USize(5419),
            ProblemResult::USize(3351),
            ProblemResult::USize(17325),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (enhancement_key, image) = Self::parse(input);
        let enhanced_image = Self::enhance_image(image, &enhancement_key, 2);
        enhanced_image.iter().filter(|pxl| **pxl).count().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (enhancement_key, image) = Self::parse(input);
        let enhanced_image = Self::enhance_image(image, &enhancement_key, 50);
        enhanced_image.iter().filter(|pxl| **pxl).count().to_result()
    }
}
