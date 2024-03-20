use aoc_lib::cartesian::{Direction, Grid, Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution22;
impl Solution22 {
    /// Create big map and parse move list
    fn parse(input: ProblemInput) -> (Grid<char>, Vec<String>) {
        let mut map = input.grid();
        map.rows = map.rows.into_iter().take_while(|r| !r.is_empty()).collect();
        let max_width = map.rows.iter().map(|r| r.len()).max().unwrap();
        map.rows.iter_mut().for_each(|r| r.resize(max_width, ' '));
        map.size = Size::new(max_width, map.rows.len());

        let lines = input.lines();
        let path = lines.last().unwrap();
        let moves = path
            .split_inclusive(['L', 'R'])
            .flat_map(|moveturn| moveturn.rsplitn(3, "").skip(1).collect_vec().into_iter().rev())
            .filter(|step| !step.is_empty())
            .map(|step| step.to_string())
            .collect_vec();

        (map, moves)
    }

    /// Indexing and positioning of the sides in the input
    /// Sample:         Real:    12
    ///           1              3
    ///         234             54
    ///          56             6             
    fn offsets(sample: bool) -> [(usize, usize); 6] {
        if sample {
            [(0, 2), (1, 0), (1, 1), (1, 2), (2, 2), (2, 3)]
        } else {
            [(0, 1), (0, 2), (1, 1), (2, 1), (2, 0), (3, 0)]
        }
    }

    /// Create 6 grids for each side based on offsets
    fn split_grids(map: &Grid<char>, size: usize, sample: bool) -> [Grid<char>; 6] {
        Self::offsets(sample).map(|(oj, oi)| {
            Grid::new(
                map.rows
                    .iter()
                    .skip(oj * size)
                    .take(size)
                    .map(|r| r.iter().copied().skip(oi * size).take(size).collect())
                    .collect(),
            )
        })
    }
}

impl Solution for Solution22 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(6032),
            ProblemResult::USize(80392),
            ProblemResult::USize(5031),
            ProblemResult::USize(19534),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (map, path) = Self::parse(input);

        // Move to start
        let mut idx = Index::new(0, 0);
        while *map.get(&idx) == ' ' {
            idx = idx.advance(Direction::East);
        }

        // Follow path
        let mut dir = Direction::East;
        for step in path {
            match step.parse::<u16>() {
                Ok(tiles) => {
                    let mut moves = 0;
                    let mut last_valid = idx;
                    while moves < tiles {
                        let next = idx.advance_wrap(dir, map.size);
                        match map.get(&next) {
                            '.' => {
                                idx = next;
                                last_valid = idx;
                                moves += 1;
                            }
                            ' ' => {
                                idx = next;
                            }
                            '#' => {
                                idx = last_valid;
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Err(_) => match step.as_str() {
                    "L" => dir = dir.left(),
                    "R" => dir = dir.right(),
                    _ => unreachable!(),
                },
            }
        }

        // Compute password
        let pwd = (1000 * (idx.j + 1)) + (4 * (idx.i + 1)) + (dir as usize + 3) % 4;
        pwd.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        // Create grids
        let (whole_map, path) = Self::parse(input);
        let size = if is_sample { 4 } else { 50 };
        let maps = Self::split_grids(&whole_map, size, is_sample);

        // Move to start
        let mut map_idx = 0;
        let mut dir = Direction::East;
        let mut idx = Index::new(0, 0);

        // Follow path
        let last = size - 1;
        for step in path {
            match step.parse::<u16>() {
                Ok(tiles) => {
                    for _ in 0..tiles {
                        // Hardcoded edge transitions based on current side and current direction
                        let (next_map, next_idx, next_dir) = match idx.advance_check(dir, maps[map_idx].size) {
                            Some(next) => (map_idx, next, dir),
                            None if is_sample => match (map_idx, dir) {
                                (0, Direction::North) => (1, Index::new(last - idx.i, 0), Direction::South),
                                (0, Direction::East) => (5, Index::new(last, last - idx.j), Direction::West),
                                (0, Direction::South) => (3, Index::new(idx.i, 0), Direction::South),
                                (0, Direction::West) => (2, Index::new(idx.j, 0), Direction::South),
                                (1, Direction::North) => (0, Index::new(last - idx.i, 0), Direction::South),
                                (1, Direction::East) => (2, Index::new(0, idx.j), Direction::East),
                                (1, Direction::South) => (4, Index::new(last - idx.i, last), Direction::North),
                                (1, Direction::West) => (5, Index::new(last - idx.j, last), Direction::North),
                                (2, Direction::North) => (0, Index::new(0, idx.i), Direction::East),
                                (2, Direction::East) => (3, Index::new(0, idx.j), Direction::East),
                                (2, Direction::South) => (4, Index::new(0, last - idx.i), Direction::East),
                                (2, Direction::West) => (1, Index::new(last, idx.j), Direction::West),
                                (3, Direction::North) => (0, Index::new(idx.i, last), Direction::North),
                                (3, Direction::East) => (5, Index::new(last - idx.j, 0), Direction::South),
                                (3, Direction::South) => (4, Index::new(idx.i, 0), Direction::South),
                                (3, Direction::West) => (2, Index::new(last, idx.j), Direction::West),
                                (4, Direction::North) => (3, Index::new(idx.i, last), Direction::North),
                                (4, Direction::East) => (5, Index::new(0, idx.j), Direction::East),
                                (4, Direction::South) => (1, Index::new(last - idx.i, last), Direction::North),
                                (4, Direction::West) => (2, Index::new(last - idx.j, last), Direction::North),
                                (5, Direction::North) => (3, Index::new(last, last - idx.i), Direction::West),
                                (5, Direction::East) => (0, Index::new(last, last - idx.j), Direction::West),
                                (5, Direction::South) => (1, Index::new(0, last - idx.i), Direction::East),
                                (5, Direction::West) => (4, Index::new(last, idx.j), Direction::West),
                                _ => unreachable!(),
                            },
                            None => match (map_idx, dir) {
                                (0, Direction::North) => (5, Index::new(0, idx.i), Direction::East),
                                (0, Direction::East) => (1, Index::new(0, idx.j), Direction::East),
                                (0, Direction::South) => (2, Index::new(idx.i, 0), Direction::South),
                                (0, Direction::West) => (4, Index::new(0, last - idx.j), Direction::East),
                                (1, Direction::North) => (5, Index::new(idx.i, last), Direction::North),
                                (1, Direction::East) => (3, Index::new(last, last - idx.j), Direction::West),
                                (1, Direction::South) => (2, Index::new(last, idx.i), Direction::West),
                                (1, Direction::West) => (0, Index::new(last, idx.j), Direction::West),
                                (2, Direction::North) => (0, Index::new(idx.i, last), Direction::North),
                                (2, Direction::East) => (1, Index::new(idx.j, last), Direction::North),
                                (2, Direction::South) => (3, Index::new(idx.i, 0), Direction::South),
                                (2, Direction::West) => (4, Index::new(idx.j, 0), Direction::South),
                                (3, Direction::North) => (2, Index::new(idx.i, last), Direction::North),
                                (3, Direction::East) => (1, Index::new(last, last - idx.j), Direction::West),
                                (3, Direction::South) => (5, Index::new(last, idx.i), Direction::West),
                                (3, Direction::West) => (4, Index::new(last, idx.j), Direction::West),
                                (4, Direction::North) => (2, Index::new(0, idx.i), Direction::East),
                                (4, Direction::East) => (3, Index::new(0, idx.j), Direction::East),
                                (4, Direction::South) => (5, Index::new(idx.i, 0), Direction::South),
                                (4, Direction::West) => (0, Index::new(0, last - idx.j), Direction::East),
                                (5, Direction::North) => (4, Index::new(idx.i, last), Direction::North),
                                (5, Direction::East) => (3, Index::new(idx.j, last), Direction::North),
                                (5, Direction::South) => (1, Index::new(idx.i, 0), Direction::South),
                                (5, Direction::West) => (0, Index::new(idx.j, 0), Direction::South),
                                _ => unreachable!(),
                            },
                        };

                        // Hit wall
                        if *maps[next_map].get(&next_idx) == '#' {
                            break;
                        }

                        map_idx = next_map;
                        idx = next_idx;
                        dir = next_dir;
                    }
                }
                Err(_) => match step.as_str() {
                    "L" => dir = dir.left(),
                    "R" => dir = dir.right(),
                    _ => unreachable!(),
                },
            }
        }

        // Compute password
        let (oj, oi) = Self::offsets(is_sample)[map_idx];
        let pwd = (1000 * ((size * oj) + idx.j + 1)) + (4 * ((size * oi) + idx.i + 1)) + (dir as usize + 3) % 4;
        pwd.to_result()
    }
}
