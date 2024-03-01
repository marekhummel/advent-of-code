use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_lib::graph::Graph;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Quantity(u64, String);

impl From<&str> for Quantity {
    fn from(value: &str) -> Self {
        let (q, t) = value.trim().split_once(' ').unwrap();
        Quantity(q.trim().parse().unwrap(), t.trim().to_string())
    }
}

pub struct Solution14;
impl Solution14 {
    fn parse(input: ProblemInput) -> Graph<Quantity> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (lhs, rhs) = l.split_once(" => ").unwrap();
                let target = rhs.into();
                let ingredients = lhs.split(',').map(|ing| ing.into()).collect();
                (target, ingredients)
            })
            .collect()
    }

    fn compute_required_ore(graph: &Graph<Quantity>, fuel: u64) -> u64 {
        // Sort graph to find order in which to produce chemicals
        let dependencies: Graph<_> = graph
            .iter()
            .map(|(trg_quant, ings)| (&trg_quant.1, ings.iter().map(|(ing_quant, _)| &ing_quant.1).collect()))
            .collect();
        let production_order = dependencies.topo_sorting().unwrap();

        // Lookup for recipes based on target chemical
        let recipes: HashMap<_, _> = graph
            .iter()
            .map(|(trg_quant, ingredients)| (&trg_quant.1, (trg_quant, ingredients.keys().collect_vec())))
            .collect();

        // Get supplies
        let mut supplies = HashMap::from([(String::from("FUEL"), fuel)]);
        for target_chemical in production_order {
            if target_chemical == "ORE" {
                break;
            }

            // Find recipe for chemical and compute the amount this recipe has to be made
            let (recipe_outcome, ingredients) = &recipes[target_chemical];
            let required_amount = supplies.remove(target_chemical).unwrap();
            let factor = (required_amount - 1) / recipe_outcome.0 + 1;

            // Update our supplies
            for ing in ingredients {
                *supplies.entry(ing.1.clone()).or_insert(0) += ing.0 * factor;
            }
        }

        // Check how much ore should be in the supplies for one fuel.
        let ore = supplies.remove("ORE").unwrap();
        assert!(supplies.is_empty());
        ore
    }
}

impl Solution for Solution14 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U64(180697),
            ProblemResult::U64(337075),
            ProblemResult::U64(5586022),
            ProblemResult::U64(5194174),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::parse(input);
        Self::compute_required_ore(&graph, 1).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let graph = Self::parse(input);
        let total_ore = 1_000_000_000_000;

        let mut low = 1;
        let mut high = total_ore; // 1 ore for 1 fuel is best case

        // Binary search
        while high - low > 1 {
            let fuel = (low + high) / 2;

            let ore = Self::compute_required_ore(&graph, fuel);
            match ore.cmp(&total_ore) {
                Ordering::Less => low = fuel,
                Ordering::Equal => break,
                Ordering::Greater => high = fuel,
            }
        }

        low.to_result()
    }
}
