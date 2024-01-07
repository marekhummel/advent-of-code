use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use regex::Regex;

pub struct Solution09;
impl Solution09 {
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
                let reference = Self::decompress_v2(&sequence[i..i + chars]);
                decompressed += reference * repeat;
                i += chars;
            } else {
                decompressed += 1;
                i += 1;
            }
        }
        decompressed
    }
}

impl Solution for Solution09 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        input
            .lines()
            .into_iter()
            .map(|line| Self::decompress(&line).len())
            .sum::<usize>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        input
            .lines()
            .into_iter()
            .map(|line| Self::decompress_v2(&line))
            .sum::<u64>()
            .into_some()
    }
}
