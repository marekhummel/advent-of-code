use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;
use rayon::iter::*;

// Total day needs 24secs on release mode
pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let door = input.string();

    find_hashes(door)
        .take(8)
        .map(|h| h.chars().nth(5).unwrap())
        .join("")
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let door = input.string();

    let mut password = [None; 8];
    for hash in find_hashes(door) {
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

    password.map(|c| c.unwrap()).iter().join("").into()
}

const STEP_SIZE: usize = 1 << 20;

fn find_hashes(door_id: String) -> impl Iterator<Item = String> {
    (1usize..).step_by(STEP_SIZE).flat_map(move |bn| {
        (bn..bn + STEP_SIZE)
            .into_par_iter()
            .map(|n| format!("{:x}", md5::compute(format!("{door_id}{n}"))))
            .filter(|digest| digest.starts_with("00000"))
            .collect::<Vec<_>>()
    })
}
