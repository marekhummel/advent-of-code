use std::collections::{HashMap, VecDeque};

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution21;
impl Solution21 {
    fn parse(input: ProblemInput) -> Vec<(Vec<String>, Vec<String>)> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let words = l.split_whitespace().collect_vec();
                let (ingredients, allergens) = words.split(|w| *w == "(contains").collect_tuple().unwrap();

                (
                    ingredients.iter().map(|i| i.to_string()).collect(),
                    allergens
                        .iter()
                        .map(|a| a.trim_end_matches([',', ')']).to_string())
                        .collect(),
                )
            })
            .collect()
    }

    fn find_allergenic_ingredients(foods: &[(Vec<String>, Vec<String>)]) -> HashMap<&String, &String> {
        let potentials = Self::find_potential_allergenic_ingredients(foods);

        // Find allergenic ingredients by process of elimination
        let mut queue = VecDeque::from_iter(potentials);
        let mut allergenic_ingredients = HashMap::new();
        while let Some((allergen, potentials)) = queue.pop_front() {
            if let [ingredient] = potentials[..] {
                allergenic_ingredients.insert(allergen, ingredient);
                queue.iter_mut().for_each(|(_, is)| is.retain(|i| *i != ingredient));
            } else {
                queue.push_back((allergen, potentials));
            }
        }

        allergenic_ingredients
    }

    fn find_potential_allergenic_ingredients(foods: &[(Vec<String>, Vec<String>)]) -> HashMap<&String, Vec<&String>> {
        let all_ingredients = foods.iter().flat_map(|(i, _)| i).unique().collect_vec();

        let mut potentials: HashMap<_, _> = foods
            .iter()
            .flat_map(|(_, a)| a)
            .unique()
            .map(|a| (a, all_ingredients.clone()))
            .collect();

        // Reduce potential carriers to make sure ingredient is everywhere present where the allergen is named.
        for (ingredients, allergens) in foods {
            for allergen in allergens {
                potentials
                    .get_mut(&allergen)
                    .unwrap()
                    .retain(|i| ingredients.contains(i));
            }
        }

        potentials
    }
}

impl Solution for Solution21 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let foods = Self::parse(input);
        let potentials = Self::find_potential_allergenic_ingredients(&foods);

        let all_ingredients = foods.iter().flat_map(|(i, _)| i).unique();
        let no_allergen_ingredients = all_ingredients
            .filter(|i| !potentials.values().any(|ci| ci.contains(i)))
            .collect_vec();

        let appearences: usize = foods
            .iter()
            .map(|(is, _)| is.iter().filter(|i| no_allergen_ingredients.contains(i)).count())
            .sum();

        appearences.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let foods = Self::parse(input);
        let allergenic_ingredients = Self::find_allergenic_ingredients(&foods);

        let canonical_dangerous_list = allergenic_ingredients
            .into_iter()
            .sorted_by_key(|(a, _)| *a)
            .map(|(_, i)| i)
            .join(",");

        canonical_dangerous_list.to_result()
    }
}
