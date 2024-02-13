use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution20;
impl Solution20 {
    fn parse(input: ProblemInput) -> Vec<(u32, u32)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (low, high) = l.split_once('-').unwrap();
                (low.trim().parse().unwrap(), high.trim().parse().unwrap())
            })
            .collect_vec()
    }

    fn allowed_ips(blacklist_ranges: Vec<(u32, u32)>, max: u32) -> Vec<(u32, u32)> {
        let mut ips = blacklist_ranges
            .iter()
            .flat_map(|(l, h)| [*l, *h])
            .sorted()
            .collect_vec();
        ips.push(max);
        let mut blacklist = vec![true; ips.len()];

        for (low, high) in blacklist_ranges {
            let li = ips.binary_search(&low).unwrap();
            let hi = ips.binary_search(&high).unwrap();
            blacklist.iter_mut().take(hi).skip(li).for_each(|b| *b = false);
        }

        blacklist
            .into_iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, b1), _)| *b1)
            .map(|((i, _), (j, _))| (ips[i] + 1, ips[j] - if ips[j] != max { 1 } else { 0 }))
            .filter(|(l, h)| l <= h)
            .collect_vec()
    }
}

impl Solution for Solution20 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let ranges = Self::parse(input);
        let whitelist = Self::allowed_ips(ranges, u32::MAX);
        whitelist[0].0.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let ranges = Self::parse(input);
        let whitelist = Self::allowed_ips(ranges, u32::MAX);
        whitelist.into_iter().map(|(l, h)| h - l + 1).sum::<u32>().to_result()
    }
}
