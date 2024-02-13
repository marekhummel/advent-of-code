use std::collections::HashMap;

use aoc_lib::math::lcm;
use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
pub struct Solution08;

impl Solution08 {
    fn parse(&self, input: ProblemInput) -> (String, HashMap<String, HashMap<char, String>>) {
        let mut lines = input.lines().into_iter();

        let instruction = String::from(lines.next().unwrap().trim());

        let mut network = HashMap::new();
        for node in lines.skip(1) {
            let (name, next) = node.split_once('=').unwrap();
            let (left, right) = next.trim_matches(&['(', ')', ' '][..]).split_once(',').unwrap();

            let continuation = HashMap::from([('L', String::from(left.trim())), ('R', String::from(right.trim()))]);
            network.insert(String::from(name.trim()), continuation);
        }

        (instruction, network)
    }
}

impl Solution for Solution08 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (instruction, network) = self.parse(input);

        let mut node = String::from("AAA");
        let mut steps = 0;
        loop {
            for inst in instruction.chars() {
                node = network[&node][&inst].clone();
                steps += 1;
                if node == "ZZZ" {
                    return steps.to_result();
                }
            }
        }
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (instruction, network) = self.parse(input);

        // define starting nodes and count when each starting node reaches an end state
        let starting_nodes = network.keys().filter(|s| s.ends_with('A')).cloned().collect_vec();
        let mut finishes = HashMap::new();

        // Compute instructions
        let mut nodes = starting_nodes.iter().cloned().collect_vec();
        let mut steps = 0u64;

        // Loop until all starting nodes have finished once
        while finishes.len() < starting_nodes.len() {
            // Loop through entire instruction
            for inst in instruction.chars() {
                nodes = nodes.iter().map(|node| network[node][&inst].clone()).collect_vec();
                steps += 1;

                // Store step count for all starting nodes that now have reached an end state
                nodes
                    .iter()
                    .zip_eq(starting_nodes.iter())
                    .filter(|(n, _)| n.ends_with('Z'))
                    .for_each(|(_, sn)| finishes.entry(sn).or_insert(Vec::new()).push(steps))
            }
        }

        // Compute all combinations of finishes and find the lowest LCM
        finishes
            .values()
            .cloned()
            .multi_cartesian_product()
            .map(|fs| lcm(&fs[..]))
            .min()
            .unwrap()
            .to_result()
    }
}
