use std::collections::{HashSet, VecDeque};

use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    pos: Index,
    dir: Direction,
}

pub struct Solution16;
impl Solution16 {
    fn traverse(grid: &Grid<char>, beam_start: Beam) -> usize {
        let mut beams: VecDeque<Beam> = VecDeque::from([beam_start.clone()]);
        let mut beam_history = HashSet::from([beam_start]);

        while let Some(b) = beams.pop_front() {
            let new_dirs = match grid.get(&b.pos) {
                '.' => vec![b.dir],
                '/' => match b.dir {
                    Direction::North => vec![Direction::East],
                    Direction::East => vec![Direction::North],
                    Direction::West => vec![Direction::South],
                    Direction::South => vec![Direction::West],
                    Direction::None => unreachable!(),
                },
                '\\' => match b.dir {
                    Direction::North => vec![Direction::West],
                    Direction::East => vec![Direction::South],
                    Direction::West => vec![Direction::North],
                    Direction::South => vec![Direction::East],
                    Direction::None => unreachable!(),
                },
                '|' => match b.dir {
                    Direction::North | Direction::South => vec![b.dir],
                    Direction::East | Direction::West => vec![Direction::North, Direction::South],
                    Direction::None => unreachable!(),
                },
                '-' => match b.dir {
                    Direction::East | Direction::West => vec![b.dir],
                    Direction::North | Direction::South => vec![Direction::East, Direction::West],
                    Direction::None => unreachable!(),
                },
                _ => unreachable!(),
            };

            for dir in new_dirs {
                if let Some(next_pos) = b.pos.advance_check(dir, grid.size) {
                    let new_beam = Beam { pos: next_pos, dir };
                    if !beam_history.contains(&new_beam) {
                        beam_history.insert(new_beam.clone());
                        beams.push_back(new_beam);
                    }
                }
            }
        }

        let energized: HashSet<Index> = beam_history.iter().map(|b| b.pos).collect();
        // grid.print(|idx, _| if energized.contains(&idx) { "#" } else { "." });
        energized.len()
    }
}

impl Solution for Solution16 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(46),
            ProblemResult::USize(7517),
            ProblemResult::USize(51),
            ProblemResult::USize(7741),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let grid = input.grid();
        let beam_start = Beam {
            pos: Index::new(0, 0),
            dir: Direction::East,
        };
        Self::traverse(&grid, beam_start).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let grid = input.grid();

        let mut start_beams = Vec::new();
        start_beams.extend((0..grid.size.height).map(|y| Beam {
            pos: Index::new(0, y),
            dir: Direction::East,
        }));
        start_beams.extend((0..grid.size.height).map(|y| Beam {
            pos: Index::new(grid.size.width - 1, y),
            dir: Direction::West,
        }));
        start_beams.extend((0..grid.size.width).map(|x| Beam {
            pos: Index::new(x, 0),
            dir: Direction::South,
        }));
        start_beams.extend((0..grid.size.width).map(|x| Beam {
            pos: Index::new(x, grid.size.height - 1),
            dir: Direction::North,
        }));

        start_beams
            .into_iter()
            .map(|start| Self::traverse(&grid, start))
            .max()
            .unwrap()
            .to_result()
    }
}
