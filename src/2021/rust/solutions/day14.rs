use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type RuleSource = (char, char);
type Polymer = HashMap<RuleSource, u64>;
type Template = (Polymer, char, char);

pub struct Solution14;
impl Solution14 {
    fn parse(input: ProblemInput) -> (Template, HashMap<RuleSource, char>) {
        let lines = input.lines();

        let template_str = &lines[0];
        let start = template_str.chars().next().unwrap();
        let end = template_str.chars().last().unwrap();
        let mut template = HashMap::new();
        for pair in template_str.chars().tuple_windows() {
            *template.entry(pair).or_default() += 1;
        }

        let rules = lines[2..]
            .iter()
            .map(|rule| {
                let (src_str, trg_str) = rule.split_once(" -> ").unwrap();
                let source = src_str.chars().collect_tuple().unwrap();
                let target = trg_str.chars().next().unwrap();
                (source, target)
            })
            .collect();

        ((template, start, end), rules)
    }

    fn step(polymer: Polymer, rules: &HashMap<RuleSource, char>) -> Polymer {
        let mut new_polymer = HashMap::new();

        for ((a, b), count) in polymer {
            if let Some(&c) = rules.get(&(a, b)) {
                // Rule found, meaning we have now for each AB, one AC and one CB
                *new_polymer.entry((a, c)).or_default() += count;
                *new_polymer.entry((c, b)).or_default() += count;
            } else {
                // No rule, we keep the AB
                *new_polymer.entry((a, b)).or_default() += count;
            }
        }

        new_polymer
    }

    fn evaluate(polymer: &Polymer, start: char, end: char) -> u64 {
        // Count elements by looking at the pairs.
        // Start and end char are not counted twice, thus the init
        let mut elements = HashMap::from([(start, 1), (end, 1)]);
        for ((a, b), count) in polymer {
            *elements.entry(*a).or_default() += count;
            *elements.entry(*b).or_default() += count;
        }

        // Div by two because all elements are counted twice
        let (least, most) = elements.values().minmax().into_option().unwrap();
        most / 2 - least / 2
    }
}

impl Solution for Solution14 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(1588),
            ProblemResult::U64(3906),
            ProblemResult::U64(2188189693529),
            ProblemResult::U64(4441317262452),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let ((template, start, end), rules) = Self::parse(input);
        let final_polymer = (0..10).fold(template, |polymer, _| Self::step(polymer, &rules));
        Self::evaluate(&final_polymer, start, end).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let ((template, start, end), rules) = Self::parse(input);
        let final_polymer = (0..40).fold(template, |polymer, _| Self::step(polymer, &rules));
        Self::evaluate(&final_polymer, start, end).to_result()
    }
}
