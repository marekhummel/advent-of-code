use aoc_lib::prelude::types::{ProblemInput, ProblemResult};
use regex::Regex;

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .map(|line| decompress(&line).len())
        .sum::<usize>()
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    input
        .lines()
        .into_iter()
        .map(|line| decompress_v2(&line))
        .sum::<u64>()
        .into()
}

fn decompress(sequence: &str) -> String {
    let marker_rgx = Regex::new(r"^\((?P<chars>\d+)x(?P<repeat>\d+)\)").unwrap();
    let mut decompressed = String::new();

    let mut i = 0;
    while i < sequence.len() {
        if let Some(captures) = marker_rgx.captures(&sequence[i..]) {
            let chars = captures.name("chars").unwrap().as_str().parse::<usize>().unwrap();
            let repeat = captures.name("repeat").unwrap().as_str().parse::<usize>().unwrap();

            i += captures.get(0).unwrap().len();
            let reference = &sequence[i..i + chars];
            decompressed.push_str(&reference.repeat(repeat));
            i += chars;
        } else {
            decompressed.push_str(&sequence[i..i + 1]);
            i += 1;
        }
    }

    decompressed
}

// Same as v1, but applies decompress on reference recursively and only counts string length instead of creating the string
fn decompress_v2(sequence: &str) -> u64 {
    let marker_rgx = Regex::new(r"^\((?P<chars>\d+)x(?P<repeat>\d+)\)").unwrap();
    let mut decompressed = 0;

    let mut i = 0;
    while i < sequence.len() {
        if let Some(captures) = marker_rgx.captures(&sequence[i..]) {
            let chars = captures.name("chars").unwrap().as_str().parse::<usize>().unwrap();
            let repeat = captures.name("repeat").unwrap().as_str().parse::<u64>().unwrap();

            i += captures.get(0).unwrap().len();
            let reference = decompress_v2(&sequence[i..i + chars]);
            decompressed += reference * repeat;
            i += chars;
        } else {
            decompressed += 1;
            i += 1;
        }
    }
    decompressed
}
