use std::collections::HashSet;

use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let components = parse(input);
    let bridges = build_bridges(&components, 0, &mut HashSet::new());

    bridges
        .into_iter()
        .max_by_key(|(_, strength)| *strength)
        .unwrap()
        .1
        .into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let components = parse(input);
    let bridges = build_bridges(&components, 0, &mut HashSet::new());

    bridges.into_iter().max().unwrap().1.into()
}

fn parse(input: ProblemInput) -> Vec<(u32, u32)> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let (left, right) = l.split_once('/').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

fn build_bridges(components: &[(u32, u32)], last_type: u32, bridge: &mut HashSet<usize>) -> Vec<(usize, u32)> {
    let mut bridges = Vec::new();
    for (i, (front, back)) in components.iter().enumerate() {
        if bridge.contains(&i) {
            continue;
        }

        if *front == last_type {
            bridge.insert(i);
            bridges.extend(build_bridges(components, *back, bridge));
            bridge.remove(&i);
        }

        if *back == last_type && front != back {
            bridge.insert(i);
            bridges.extend(build_bridges(components, *front, bridge));
            bridge.remove(&i);
        }
    }

    if bridges.is_empty() {
        vec![(
            bridge.len(),
            bridge.iter().map(|i| components[*i].0 + components[*i].1).sum::<u32>(),
        )]
    } else {
        bridges
    }
}
