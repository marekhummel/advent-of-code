use std::collections::HashSet;

use aoc_lib::grammar::{Rule, CFG};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type MRule = Rule<String, String>;

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> (Vec<String>, Vec<MRule>) {
        let lines = input.lines();

        let rules = lines
            .iter()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (src, trg) = l.split_once("=>").unwrap();
                match &Self::get_molecule_parts(trg.trim())[..] {
                    [nt1, nt2] => Rule::Iterative {
                        src: src.trim().to_string(),
                        vars: [nt1.clone(), nt2.clone()],
                    },
                    [terminal] => Rule::Unit {
                        src: src.trim().to_string(),
                        terminal: terminal.clone(),
                    },
                    vars => Rule::Extended {
                        src: src.trim().to_string(),
                        vars: vars.iter().cloned().collect_vec(),
                    },
                }
            })
            .collect_vec();

        let word = Self::get_molecule_parts(lines.last().unwrap());

        (word, rules)
    }

    fn get_molecule_parts(molecule: &str) -> Vec<String> {
        // Since terminals are multiple chars, need method to split
        let (mut parts, rem) = molecule
            .chars()
            .fold((Vec::new(), String::new()), |(mut elements, mut curr), ch| {
                if ch.is_uppercase() {
                    elements.push(curr);
                    curr = ch.to_string();
                } else {
                    curr.push(ch);
                }
                (elements, curr)
            });
        if !parts.is_empty() {
            parts.remove(0);
        }
        parts.push(rem);

        parts
    }

    fn add_terminals(rules: &mut Vec<MRule>, start: &str) {
        // Find all nonterminals in current rules
        let mut nonterminals = HashSet::new();
        nonterminals.extend(rules.iter().map(|p| p.source().to_string()));
        nonterminals.extend(rules.iter().flat_map(|p| p.nonterminals()));

        // Add terminal rules in lower case for the cyk algorithm
        for nt in nonterminals {
            if nt != start && !nt.starts_with("Sub") {
                rules.push(Rule::Unit {
                    src: nt.to_string(),
                    terminal: nt.to_lowercase(),
                })
            }
        }
    }
}

impl Solution for Solution19 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(7),
            ProblemResult::USize(576),
            ProblemResult::USize(5),
            ProblemResult::USize(207),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (molecule, rules) = Self::parse(input);

        let mut new_molecules = HashSet::new();
        for rule in rules {
            let src = rule.source();
            let positions = molecule.iter().positions(|elem| elem == src).collect_vec();
            for pos in positions {
                let new_molecule = [&molecule[0..pos], &rule.nonterminals(), &molecule[pos + 1..]].concat();
                // println!("{new_molecule:?} ({rule} on {pos})");
                new_molecules.insert(new_molecule);
            }
        }

        new_molecules.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (mut word, rules) = Self::parse(input);
        let start = "e";

        // Transform given CFG to CNF for CYK algorithm
        let mut cfg = CFG {
            rules,
            start: String::from("e"),
            sub_func: |sub_len| format!("Sub{sub_len}"),
        };
        cfg.transform_cnf();
        Self::add_terminals(&mut cfg.rules, start);

        // Transform word to lower case to work with terminals
        word = word.into_iter().map(|elem| elem.to_lowercase()).collect_vec();

        // Sample is one application short due to manual adaptation of language for start symbol
        let rules = cfg.cyk_algorithm(&word).unwrap();

        // Count all rules, except for the substitute and terminal ones as they weren't in the original
        rules
            .into_iter()
            .filter(|p| matches!(p, Rule::Iterative { src, vars: _ } if !src.starts_with("Sub")))
            .count()
            .to_result()
    }
}
