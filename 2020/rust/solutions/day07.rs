use std::collections::{HashMap, HashSet};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution07;
impl Solution07 {
    fn parse(input: ProblemInput) -> HashMap<String, Vec<(u32, String)>> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (outer, inner_str) = l.split_once(" bags contain ").unwrap();
                if inner_str == "no other bags." {
                    return (outer.to_string(), vec![]);
                }

                let inner = inner_str
                    .split(',')
                    .map(|l| {
                        let mut words = l.split_ascii_whitespace().collect_vec();
                        words.pop();
                        (words[0].parse().unwrap(), words[1..].join(" "))
                    })
                    .collect();
                (outer.to_string(), inner)
            })
            .collect()
    }

    fn count_nested(
        bag: &str,
        rules: &HashMap<String, Vec<(u32, String)>>,
        known_capacities: &mut HashMap<String, u32>, // interestingly enough, memoization is not even necessary
    ) -> u32 {
        let mut nested = 0;
        for (count, inner_bag) in &rules[&bag.to_string()] {
            let cap = known_capacities.get(inner_bag).copied().unwrap_or_else(|| {
                let cap = Self::count_nested(inner_bag, rules, known_capacities);
                known_capacities.insert(inner_bag.clone(), cap);
                cap
            });

            nested += count * (cap + 1);
        }

        nested
    }
}

impl Solution for Solution07 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rules = Self::parse(input);

        let mut can_carry_shiny_gold = HashSet::from(["shiny gold"]);
        loop {
            let before = can_carry_shiny_gold.len();

            for (bag, inner) in rules.iter() {
                if inner.iter().any(|(_, ib)| can_carry_shiny_gold.contains(ib.as_str())) {
                    can_carry_shiny_gold.insert(bag);
                }
            }

            if can_carry_shiny_gold.len() == before {
                break;
            }
        }

        can_carry_shiny_gold.remove("shiny gold");
        can_carry_shiny_gold.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let rules = Self::parse(input);
        Self::count_nested("shiny gold", &rules, &mut HashMap::new()).to_result()
    }
}
