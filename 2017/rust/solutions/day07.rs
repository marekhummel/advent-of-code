use std::collections::HashMap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{Either, Itertools};

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: u32,
    supports: Vec<String>,
}

pub struct Solution07;
impl Solution07 {
    fn parse(input: ProblemInput) -> Vec<Program> {
        // ugml (68) -> gyxo, ebii, jptl
        input
            .lines()
            .into_iter()
            .map(|l| {
                let words = l.split_whitespace().collect_vec();
                let name = words[0].to_string();
                let weight = words[1][1..words[1].len() - 1].parse().unwrap();
                let supports = if words.len() > 2 {
                    words[3..]
                        .iter()
                        .map(|s| s.trim_end_matches(',').to_string())
                        .collect_vec()
                } else {
                    vec![]
                };

                Program { name, weight, supports }
            })
            .collect()
    }

    fn find_root(programs: &[Program]) -> Program {
        let mut root = programs[0].clone();
        while let Some(parent) = programs.iter().find(|p| p.supports.contains(&root.name)) {
            root = parent.clone();
        }
        root
    }

    fn find_unbalanced(current: &str, programs: &HashMap<String, Program>) -> Either<(String, u32), u32> {
        // No children, return balanced
        let prog = &programs[current];
        if prog.supports.is_empty() {
            return Either::Right(prog.weight);
        }

        let children = prog
            .supports
            .iter()
            .map(|s| (s, Self::find_unbalanced(s, programs)))
            .collect_vec();

        // Some children are unbalanced, return
        if let Some((_, unbalanced)) = children.iter().find(|(_, c)| c.is_left()) {
            unbalanced.clone()
        } else {
            let weights = children
                .into_iter()
                .map(|(c, u)| (c, *u.as_ref().right().unwrap()))
                .collect_vec();

            if weights.iter().map(|(_, w)| w).all_equal() {
                // Weights of children all equal, hence balanced
                Either::Right(prog.weight + weights.iter().map(|(_, w)| w).sum::<u32>())
            } else {
                // Unbalanced program found
                let groups = weights.into_iter().into_group_map_by(|(_, w)| *w);
                assert_eq!(groups.len(), 2, "Tower too unbalanced");

                let (correct, wrong): (Vec<_>, Vec<_>) = groups.values().partition(|c| c.len() > 1);
                let correct_total_weight = correct[0][0].1; // first partition has 1 element, but take any of those cause weight is equal
                let (unbalanced_prog, wrong_total_weight) = wrong[0][0];
                let weight_delta = correct_total_weight as i32 - wrong_total_weight as i32;
                let corrected_weight = programs[&unbalanced_prog.to_string()].weight as i32 + weight_delta;

                Either::Left((unbalanced_prog.to_string(), corrected_weight as u32))
            }
        }
    }
}

impl Solution for Solution07 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::String("tknk".to_string()),
            ProblemResult::String("aapssr".to_string()),
            ProblemResult::U32(60),
            ProblemResult::U32(1458),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let programs = Self::parse(input);
        Self::find_root(&programs).name.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let programs = Self::parse(input);

        let root = Self::find_root(&programs).name;
        let lookup = programs.into_iter().map(|p| (p.name.clone(), p)).collect();
        let unbalanced = Self::find_unbalanced(&root, &lookup);

        unbalanced.left().unwrap().1.to_result()
    }
}
