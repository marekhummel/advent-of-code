use std::collections::{HashMap, HashSet};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution14;
impl Solution14 {
    fn find_passkeys(salt: &str, min_keys: usize, key_stretches: usize) -> HashSet<usize> {
        let mut passkeys = HashSet::new();
        let mut threes = HashMap::new();
        let mut min_n_limit = 1;

        for n in 1usize.. {
            let digest = Self::key_stretching(format!("{}{}", salt, n), key_stretches);

            // Collect characters and their counts
            let char_groups = digest
                .chars()
                .group_by(|&c| c)
                .into_iter()
                .map(|(c, grp)| (c, grp.count()))
                .collect_vec();

            // Add n to respective char entry if it has a triple
            for (c, count) in char_groups.iter() {
                if *count >= 3 {
                    threes.entry(*c).or_insert_with(Vec::new).push(n);
                    if passkeys.len() <= min_keys {
                        // Increase n limit to make sure we fetch all keys in order.
                        min_n_limit = n + 1000;
                    }
                    break;
                }
            }

            // Find quintuples to verify keys
            let fives = char_groups.iter().filter(|(_, count)| *count >= 5).collect_vec();
            for (c, _) in fives {
                if let Some(tns) = threes.get(c) {
                    for tn in tns.iter() {
                        if n - tn <= 1000 && n != *tn {
                            passkeys.insert(*tn);
                        }
                    }
                }
            }

            // Return passkeys set if all n prior to completing min count have been verified
            if passkeys.len() >= min_keys && n > min_n_limit {
                return passkeys;
            }
        }

        unreachable!()
    }

    fn key_stretching(key: String, repeats: usize) -> String {
        let mut digest = key;
        for _ in 0..repeats {
            digest = format!("{:x}", md5::compute(digest));
        }

        digest
    }
}

impl Solution for Solution14 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let salt = input.string();
        let n = 64;
        let passkeys = Self::find_passkeys(salt.trim(), n, 1);

        passkeys.into_iter().sorted().nth(n - 1).unwrap().to_result()
    }

    // Takes about a minute, both in sample and real
    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let salt = input.string();
        let n = 64;
        let passkeys = Self::find_passkeys(salt.trim(), n, 2017);

        passkeys.into_iter().sorted().nth(n - 1).unwrap().to_result()
    }
}
