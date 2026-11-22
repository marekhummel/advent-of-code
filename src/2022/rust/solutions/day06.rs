use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    find_marker(input.string(), 4).into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    find_marker(input.string(), 14).into()
}

fn find_marker(datastream_str: String, length: usize) -> usize {
    let datastream = datastream_str.chars().collect_vec();

    for (i, window) in datastream.windows(length).enumerate() {
        if window.iter().unique().count() == length {
            return i + length;
        }
    }

    unreachable!()
}
