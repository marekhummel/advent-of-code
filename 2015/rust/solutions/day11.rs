use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution11;
impl Solution11 {
    fn parse(input: ProblemInput) -> Vec<u8> {
        input.string().chars().map(|c| c as u8 - b'a').collect()
    }

    fn iterate_next(pwd: &[u8]) -> Vec<u8> {
        let mut current = pwd.to_vec();
        loop {
            current = Self::inc(&current);
            if Self::pwd_is_valid(&current) {
                break;
            }
        }
        current
    }

    fn inc(pwd: &[u8]) -> Vec<u8> {
        // Catch invalid chars right here
        if pwd.iter().any(|d| *d == 8 || *d == 11 || *d == 14) {
            let mut new_pwd = vec![0; 8];
            for (d, nd) in pwd.iter().zip_eq(new_pwd.iter_mut()) {
                *nd = *d;
                if *d == 8 || *d == 11 || *d == 14 {
                    *nd += 1;
                    break;
                }
            }
            return new_pwd;
        }

        // Just inc by one
        let mut val = pwd.iter().fold(0u64, |v, c| v * 26 + *c as u64);
        val += 1;

        let mut new_pwd = vec![0; 8];
        for d in new_pwd.iter_mut().rev() {
            *d = (val % 26) as u8;
            val /= 26;
        }

        new_pwd
    }

    fn pwd_is_valid(pwd: &[u8]) -> bool {
        let valid_chars = pwd.iter().all(|d| *d != 8 && *d != 11 && *d != 14);
        let has_straight = pwd
            .iter()
            .tuple_windows()
            .map(|(c, nc)| *nc as i8 - *c as i8)
            .group_by(|delta| *delta)
            .into_iter()
            .any(|(d, grp)| d == 1 && grp.count() >= 2);

        let has_two_pairs = pwd
            .iter()
            .dedup_with_count()
            .filter(|(count, _)| *count >= 2)
            .map(|(_, c)| c)
            .unique()
            .count()
            >= 2;

        valid_chars && has_straight && has_two_pairs
    }

    fn pwd_to_string(pwd: &[u8]) -> String {
        pwd.iter().map(|c| (c + b'a') as char).join("")
    }
}

impl Solution for Solution11 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pwd = Self::parse(input);
        let new_pwd = Self::iterate_next(&pwd);
        Self::pwd_to_string(&new_pwd).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let pwd = Self::parse(input);
        let new_pwd = Self::iterate_next(&pwd);
        let new_new_pwd = Self::iterate_next(&new_pwd);
        Self::pwd_to_string(&new_new_pwd).to_result()
    }
}
