use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use rayon::iter::*;
pub struct Solution05;
impl Solution05 {
    const STEP_SIZE: usize = 1 << 20;

    fn find_hashes(door_id: String) -> impl Iterator<Item = String> {
        (1usize..).step_by(Self::STEP_SIZE).flat_map(move |bn| {
            (bn..bn + Self::STEP_SIZE)
                .into_par_iter()
                .map(|n| format!("{:x}", md5::compute(format!("{door_id}{n}"))))
                .filter(|digest| digest.starts_with("00000"))
                .collect::<Vec<_>>()
        })
    }
}

// Total day needs 24secs on release mode
impl Solution for Solution05 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let door = input.string();

        Self::find_hashes(door)
            .take(8)
            .map(|h| h.chars().nth(5).unwrap())
            .join("")
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let door = input.string();

        let mut password = [None; 8];
        for hash in Self::find_hashes(door) {
            let (pos_c, c) = hash.chars().skip(5).take(2).collect_tuple().unwrap();
            let pos = u32::from_str_radix(pos_c.to_string().as_str(), 16).unwrap() as usize;
            if pos > 7 || password[pos].is_some() {
                continue;
            }

            password[pos] = Some(c);
            if password.iter().all(|c| c.is_some()) {
                break;
            }
        }

        password.map(|c| c.unwrap()).iter().join("").to_result()
    }
}
