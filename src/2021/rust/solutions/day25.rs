use aoc_lib::cartesian::{Direction, Grid};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution25;
impl Solution25 {
    fn step(map: Grid<char>) -> Grid<char> {
        let mut current = map;

        for (herd, dir) in [('>', Direction::East), ('v', Direction::South)] {
            let mut new_map = Grid::empty(current.size, '.');
            for (idx, &sc) in current.enumerate() {
                if sc == herd {
                    let nb = idx.advance_wrap(dir, current.size);
                    match current.get(&nb) {
                        '.' => new_map.set(&nb, herd),
                        _ => new_map.set(&idx, herd),
                    }
                } else if sc != '.' {
                    new_map.set(&idx, sc)
                }
            }
            current = new_map;
        }

        current
    }
}

impl Solution for Solution25 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(58),
            ProblemResult::U32(305),
            ProblemResult::NoPartTwo,
            ProblemResult::NoPartTwo,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut map = input.grid();

        for step in 1u32.. {
            let next = Self::step(map.clone());
            if next == map {
                return step.to_result();
            }
            map = next;
        }

        unreachable!()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        ProblemResult::NoPartTwo
    }
}
