use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use itertools::Itertools;

// Can be implemented on strings as well, but using bools improves performance by x10
pub fn part1(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let disk_size = if is_sample { 20 } else { 272 };
    let data = create_data(parse(input), disk_size);
    let checksum = checksum(data, disk_size);

    format(checksum).into()
}

pub fn part2(input: ProblemInput, is_sample: bool) -> ProblemResult {
    let disk_size = if is_sample { 20 } else { 35651584 };
    let data = create_data(parse(input), disk_size);
    let checksum = checksum(data, disk_size);

    format(checksum).into()
}

fn parse(input: ProblemInput) -> Vec<bool> {
    input.string().chars().parsed::<u8>().map(|c| c == 1).collect_vec()
}

fn create_data(init: Vec<bool>, min_length: usize) -> Vec<bool> {
    let mut a = init;
    a.reserve(min_length * 2);

    while a.len() < min_length {
        let b = a.iter().rev().map(|c| !c).collect_vec();
        a = [a, vec![false], b].concat();
    }

    a
}

fn checksum(data: Vec<bool>, length: usize) -> Vec<bool> {
    let mut checksum = data;
    checksum.truncate(length);
    loop {
        checksum = checksum.into_iter().tuples().map(|(c1, c2)| !(c1 ^ c2)).collect_vec();
        if checksum.len() & 1 == 1 {
            break;
        }
    }

    checksum
}

fn format(data: Vec<bool>) -> String {
    data.into_iter().map(|c| if c { "1" } else { "0" }).join("")
}
