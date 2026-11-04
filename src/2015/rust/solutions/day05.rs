use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
pub struct Solution05;

impl Solution05 {
    fn is_nice1(string: &str) -> bool {
        let three_vowels = string.chars().filter(|c| "aeiou".contains(*c)).count() >= 3;
        let double_letter = string.chars().tuple_windows().any(|(f, s)| f == s);
        let no_bad_substrings = ["ab", "cd", "pq", "xy"].into_iter().all(|ss| !string.contains(ss));

        three_vowels && double_letter && no_bad_substrings
    }

    fn is_nice2(string: &str) -> bool {
        let char_pairs: Vec<(char, char)> = string.chars().tuple_windows().collect_vec();
        let grouped_pairs = char_pairs
            .into_iter()
            .enumerate()
            .sorted_by_key(|(_, tpl)| *tpl)
            .group_by(|(_, tpl)| *tpl);
        let double_pair = grouped_pairs
            .into_iter()
            .map(|(_, group)| group.map(|(i, _)| i).collect_vec())
            .filter(|group| group.len() > 1)
            .flat_map(|group| group.into_iter().tuple_combinations())
            .any(|(first, second)| second - first > 1);

        let double_spaced_letter = string.chars().zip(string.chars().skip(2)).any(|(f, s)| f == s);

        double_pair && double_spaced_letter
    }
}

impl Solution for Solution05 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .into_iter()
            .filter(|s| Self::is_nice1(s))
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        input
            .lines()
            .into_iter()
            .filter(|s| Self::is_nice2(s))
            .count()
            .to_result()
    }
}
