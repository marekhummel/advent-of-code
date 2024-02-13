use std::collections::{HashSet, VecDeque};

use aoc_lib::cartesian::{Direction, Index, Size};
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution17;
impl Solution17 {}

struct Rooms {
    passcode: String,
    size: Size,
    vault: Index,
}

impl Rooms {
    fn format_path(path: &[Direction]) -> String {
        path.iter()
            .map(|dir| match dir {
                Direction::North => 'U',
                Direction::East => 'R',
                Direction::West => 'L',
                Direction::South => 'D',
                Direction::None => '_',
            })
            .join("")
    }

    fn compute_md5(&self, path: &[Direction]) -> String {
        let key = format!("{}{}", self.passcode, Self::format_path(path));
        format!("{:x}", md5::compute(key))
    }

    fn open_doors(&self, path: &[Direction]) -> HashSet<Direction> {
        let key = &self.compute_md5(path)[0..4];
        let dirs = [Direction::North, Direction::South, Direction::West, Direction::East];

        dirs.into_iter()
            .zip_eq(key.chars())
            .filter(|(_, k)| "bcdef".contains(|c| c == *k))
            .map(|(d, _)| d)
            .collect()
    }

    fn paths_to_vault(&self) -> PathsIterator {
        PathsIterator {
            rooms: self,
            queue: VecDeque::from([(Index { i: 0, j: 0 }, vec![])]),
        }
    }
}

type Path = Vec<Direction>;

struct PathsIterator<'a> {
    rooms: &'a Rooms,
    queue: VecDeque<(Index, Path)>,
}

impl<'a> Iterator for PathsIterator<'a> {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((pos, path)) = self.queue.pop_front() {
            if pos == self.rooms.vault {
                return Some(path);
            }

            let open_doors = self.rooms.open_doors(&path);
            for dir in open_doors {
                if let Some(next_pos) = pos.advance_check(dir, self.rooms.size) {
                    self.queue.push_back((next_pos, [path.clone(), vec![dir]].concat()))
                }
            }
        }

        None
    }
}

impl Solution for Solution17 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rooms = Rooms {
            passcode: input.string(),
            size: Size { width: 4, height: 4 },
            vault: Index { i: 3, j: 3 },
        };

        Rooms::format_path(&rooms.paths_to_vault().next().unwrap()).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rooms = Rooms {
            passcode: input.string(),
            size: Size { width: 4, height: 4 },
            vault: Index { i: 3, j: 3 },
        };

        rooms.paths_to_vault().map(|p| p.len()).max().unwrap().to_result()
    }
}
