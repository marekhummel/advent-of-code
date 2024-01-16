use std::collections::HashSet;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::iproduct;

pub struct Solution03;
impl Solution03 {
    fn parse(input: ProblemInput) -> Vec<(u32, usize, usize, usize, usize)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (id, dims) = l.split_once('@').unwrap();
                let (topright, size) = dims.split_once(':').unwrap();
                let (x, y) = topright.split_once(',').unwrap();
                let (w, h) = size.split_once('x').unwrap();

                (
                    id.strip_prefix('#').unwrap().trim().parse().unwrap(),
                    x.trim().parse().unwrap(),
                    y.trim().parse().unwrap(),
                    w.trim().parse().unwrap(),
                    h.trim().parse().unwrap(),
                )
            })
            .collect()
    }

    fn claim_fabric(fabric: &mut [Vec<HashSet<u32>>], claims: &[(u32, usize, usize, usize, usize)]) {
        for (id, cx, cy, cw, ch) in claims.iter().copied() {
            for (x, y) in iproduct!(cx..cx + cw, cy..cy + ch) {
                fabric[y][x].insert(id);
            }
        }
    }
}

impl Solution for Solution03 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let claims = Self::parse(input);
        let mut fabric = vec![vec![HashSet::new(); 1000]; 1000];
        Self::claim_fabric(&mut fabric, &claims);

        fabric
            .into_iter()
            .flatten()
            .filter(|ids| ids.len() > 1)
            .count()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let claims = Self::parse(input);
        let mut fabric = vec![vec![HashSet::new(); 1000]; 1000];
        Self::claim_fabric(&mut fabric, &claims);

        let overlaps: HashSet<_> = fabric
            .into_iter()
            .flatten()
            .filter(|ids| ids.len() > 1)
            .flatten()
            .collect();

        claims
            .into_iter()
            .find(|(id, ..)| !overlaps.contains(id))
            .unwrap()
            .0
            .into_some()
    }
}