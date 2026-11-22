use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    if is_sample {
        return ProblemResult::NoSample;
    }

    let (low, high) = input.string().split('-').parsed::<u32>().collect_tuple().unwrap();

    let mut total = 0;
    for pwd_num in low..=high {
        let pwd = pwd_num.to_string();
        let non_decreasing = pwd.bytes().tuple_windows().all(|(d1, d2)| d1 <= d2);
        let double = pwd.bytes().chunk_by(|d| *d).into_iter().any(|(_, g)| g.count() > 1);

        if non_decreasing && double {
            total += 1;
        }
    }

    total.into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    if is_sample {
        return ProblemResult::NoSample;
    }

    let (low, high) = input.string().split('-').parsed::<u32>().collect_tuple().unwrap();

    let mut total = 0;
    for pwd_num in low..=high {
        let pwd = pwd_num.to_string();
        let non_decreasing = pwd.bytes().tuple_windows().all(|(d1, d2)| d1 <= d2);
        let double = pwd.bytes().chunk_by(|d| *d).into_iter().any(|(_, g)| g.count() == 2);

        if non_decreasing && double {
            total += 1;
        }
    }

    total.into()
}
