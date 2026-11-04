use std::collections::{HashSet, VecDeque};

use aoc_lib::cartesian::Grid;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution11;
impl Solution11 {
    fn step(octopi: &mut Grid<u8>) -> usize {
        octopi.iter_mut().for_each(|oct| *oct += 1);

        let mut queue = octopi
            .enumerate()
            .filter_map(|(idx, energy)| (*energy > 9).then_some(idx))
            .collect::<VecDeque<_>>();
        let mut flashed = HashSet::new();
        while let Some(octopus) = queue.pop_front() {
            if !flashed.contains(&octopus) {
                flashed.insert(octopus);
                for nb in octopus.moore_neighbors(octopi.size) {
                    let energy = octopi.get_mut(&nb);
                    *energy += 1;
                    if *energy > 9 {
                        queue.push_back(nb);
                    }
                }
            }
        }

        flashed.iter().for_each(|f| octopi.set(f, 0));
        flashed.len()
    }
}

impl Solution for Solution11 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(1656),
            ProblemResult::USize(1601),
            ProblemResult::I32(195),
            ProblemResult::I32(368),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut octopi = input.grid().map_elements(|c| *c as u8 - b'0');
        let total_flashes = (0..100).fold(0, |flashes, _| flashes + Self::step(&mut octopi));
        total_flashes.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut octopi = input.grid().map_elements(|c| *c as u8 - b'0');
        let all_flash = (1..).find(|_| Self::step(&mut octopi) == octopi.size.area()).unwrap();
        all_flash.to_result()
    }
}
