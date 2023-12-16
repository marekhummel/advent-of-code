#![allow(unused_variables)]
use std::collections::{HashSet, VecDeque};

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult};
use aoc_lib::util::{Direction, Position};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    pos: Position,
    dir: Direction,
}

pub struct Solution16;
impl Solution16 {
    fn parse(input: ProblemInput) -> (Vec<Vec<char>>, (usize, usize)) {
        (input.grid(), input.grid_size())
    }

    fn traverse(grid: &Vec<Vec<char>>, beam_start: Beam, width: usize, height: usize) -> usize {
        let mut beams: VecDeque<Beam> = VecDeque::from([beam_start.clone()]);
        let mut beam_history = HashSet::from([beam_start.clone()]);

        while let Some(b) = beams.pop_front() {
            let new_dirs = match grid[b.pos.y][b.pos.x] {
                '.' => vec![b.dir],
                '/' => match b.dir {
                    Direction::North => vec![Direction::East],
                    Direction::East => vec![Direction::North],
                    Direction::West => vec![Direction::South],
                    Direction::South => vec![Direction::West],
                },
                '\\' => match b.dir {
                    Direction::North => vec![Direction::West],
                    Direction::East => vec![Direction::South],
                    Direction::West => vec![Direction::North],
                    Direction::South => vec![Direction::East],
                },
                '|' => match b.dir {
                    Direction::North | Direction::South => vec![b.dir],
                    Direction::East | Direction::West => vec![Direction::North, Direction::South],
                },
                '-' => match b.dir {
                    Direction::East | Direction::West => vec![b.dir],
                    Direction::North | Direction::South => vec![Direction::East, Direction::West],
                },
                _ => unreachable!(),
            };

            for dir in new_dirs {
                if let Some(next_pos) = b.pos.advance(dir, width, height) {
                    let new_beam = Beam { pos: next_pos, dir };
                    if !beam_history.contains(&new_beam) {
                        beam_history.insert(new_beam.clone());
                        beams.push_back(new_beam);
                    }
                }
            }
        }

        let energized: HashSet<Position> = beam_history.iter().map(|b| b.pos).collect();
        energized.len()
        // for y in 0..height {
        //     for x in 0..width {
        //         if energized.contains(&Position { x, y }) {
        //             print!("#");
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!()
        // }
    }
}

impl Solution for Solution16 {
    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (grid, (height, width)) = Self::parse(input);
        let beam_start = Beam {
            pos: Position { x: 0, y: 0 },
            dir: Direction::East,
        };
        Some(Self::traverse(&grid, beam_start, width, height).into())
    }

    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult> {
        let (grid, (height, width)) = Self::parse(input);

        let mut start_beams = Vec::new();
        start_beams.extend((0..height).map(|y| Beam {
            pos: Position { x: 0, y },
            dir: Direction::East,
        }));
        start_beams.extend((0..height).map(|y| Beam {
            pos: Position { x: width - 1, y },
            dir: Direction::West,
        }));
        start_beams.extend((0..width).map(|x| Beam {
            pos: Position { x, y: 0 },
            dir: Direction::South,
        }));
        start_beams.extend((0..width).map(|x| Beam {
            pos: Position { x, y: height - 1 },
            dir: Direction::North,
        }));

        Some(
            start_beams
                .into_iter()
                .map(|start| Self::traverse(&grid, start, width, height))
                .max()
                .unwrap()
                .into(),
        )
    }
}
