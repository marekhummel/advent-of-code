use aoc_lib::cartesian::{Direction, Grid};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let mut map = input.grid();

    for step in 1u32.. {
        let next = step(map.clone());
        if next == map {
            return step.into();
        }
        map = next;
    }

    unreachable!()
}

pub fn part2(_input: ProblemInput, _is_sample: bool) -> ProblemResult {
    ProblemResult::NoPartTwo
}

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
