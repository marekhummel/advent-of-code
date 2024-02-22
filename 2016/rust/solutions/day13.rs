use std::collections::{HashSet, VecDeque};

use aoc_lib::cartesian::{Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution13;
impl Solution13 {
    fn is_path(pos: &Index, fav: usize) -> bool {
        let n = pos.i * pos.i + 3 * pos.i + 2 * pos.i * pos.j + pos.j + pos.j * pos.j + fav;
        n.count_ones() & 1 == 0
    }

    fn find(fav: usize, start: Index, goal: Index) -> u32 {
        let size = Size::square(usize::MAX);
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
        let size = Size::square(usize::MAX);
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
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(11),
            ProblemResult::U32(90),
            ProblemResult::USize(151),
            ProblemResult::USize(135),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let fav = input.string().parse().unwrap();
        let start = Index::new(1, 1);
        let goal = if is_sample {
            Index::new(7, 4)
        } else {
            Index::new(31, 39)
        };

        Self::find(fav, start, goal).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let fav = input.string().parse().unwrap();
        let start = Index::new(1, 1);

        Self::spread(fav, start, 50).to_result()
    }
}
