use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .map(pass_to_seatid)
        .max()
        .unwrap()
        .into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    if is_sample {
        return ProblemResult::NoSample;
    }

    let ids = input.lines().into_iter().map(pass_to_seatid).collect_vec();
    let (min, max) = ids.iter().minmax().into_option().unwrap();

    (*min..=*max)
        .find(|id| ids.contains(&(id - 1)) && !ids.contains(id) && ids.contains(&(id + 1)))
        .unwrap()
        .into()
}

fn pass_to_seatid(pass: String) -> u32 {
    let (row_str, seat_str) = pass.split_at(7);
    let row = row_str.bytes().map(|c| c == b'B').fold(0, |r, b| (r << 1) + b as u32);
    let seat = seat_str.bytes().map(|c| c == b'R').fold(0, |s, r| (s << 1) + r as u32);
    row * 8 + seat
}
