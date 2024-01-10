use std::collections::{HashSet, VecDeque};

use aoc_lib::cartesian::{Index, Size};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution13;
impl Solution13 {
    fn is_path(pos: &Index, fav: usize) -> bool {
        let n = pos.i * pos.i + 3 * pos.i + 2 * pos.i * pos.j + pos.j + pos.j * pos.j + fav;
        n.count_ones() & 1 == 0
    }

    fn find(fav: usize, start: Index, goal: Index) -> u32 {
        let size = Size {
            width: usize::MAX,
            height: usize::MAX,
        };
        let mut seen = HashSet::from([(start)]);
        let mut queue = VecDeque::from([(0u32, start)]);

        while let Some((steps, pos)) = queue.pop_front() {
            if pos == goal {
                return steps;
            }

            for next_pos in pos.von_neumann_neighbors(size) {
                if Self::is_path(&next_pos, fav) && !seen.contains(&next_pos) {
                    seen.insert(next_pos);
                    queue.push_back((steps + 1, next_pos));
                }
            }
        }

        unreachable!()
    }

    fn spread(fav: usize, start: Index, max_steps: u32) -> usize {
        let size = Size {
            width: usize::MAX,
            height: usize::MAX,
        };
        let mut seen = HashSet::from([(start)]);
        let mut queue = VecDeque::from([(0u32, start)]);

        while let Some((steps, pos)) = queue.pop_front() {
            if steps >= max_steps {
                continue;
            }

            for next_pos in pos.von_neumann_neighbors(size) {
                if Self::is_path(&next_pos, fav) && !seen.contains(&next_pos) {
                    seen.insert(next_pos);
                    queue.push_back((steps + 1, next_pos));
                }
            }
        }

        seen.len()
    }
}

impl Solution for Solution13 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        let fav = input.string().parse().unwrap();
        let start = Index { i: 1, j: 1 };
        let goal = if is_sample {
            Index { i: 7, j: 4 }
        } else {
            Index { i: 31, j: 39 }
        };

        Self::find(fav, start, goal).into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let fav = input.string().parse().unwrap();
        let start = Index { i: 1, j: 1 };

        Self::spread(fav, start, 50).into_some()
    }
}
