use aoc_lib::grammar::{Rule, CFG};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> (CFG<u16, char>, Vec<Vec<char>>) {
        let lines = input.lines();
        let (cfg, word_strs) = lines.split(|l| l.is_empty()).collect_tuple().unwrap();

        let rules = cfg
            .iter()
            .flat_map(|rule_str| {
                let (lhs, rhss) = rule_str.split_once(": ").unwrap();
                if rhss.contains('"') {
                    let terminal = rhss.chars().nth(1).unwrap();
                    vec![Rule::Unit {
                        src: lhs.parse().unwrap(),
                        terminal,
                    }]
                } else {
                    rhss.split('|')
                        .map(|rhs| {
                            let nts = rhs.split_ascii_whitespace().parsed().collect_vec();

                            if nts.len() == 2 {
                                Rule::Iterative {
                                    src: lhs.parse().unwrap(),
                                    vars: nts.try_into().unwrap(),
                                }
                            } else {
                                Rule::Extended {
                                    src: lhs.parse().unwrap(),
                                    vars: nts,
                                }
                            }
                        })
                        .collect_vec()
                }
            })
            .collect();

        let words = word_strs.iter().map(|word| word.chars().collect()).collect();

        (
            CFG {
                rules,
                start: 0,
                sub_func: |sub_len| 10000 + sub_len as u16,
            },
            words,
        )
    }
}

impl Solution for Solution19 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(2),
            ProblemResult::USize(208),
            ProblemResult::USize(12),
            ProblemResult::USize(316),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (mut cfg, words) = Self::parse(input);
        cfg.transform_cnf();

        let matches = words
            .into_par_iter()
            .filter(|word| cfg.cyk_algorithm(word).is_some())
            .count();

        matches.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (mut cfg, words) = Self::parse(input);
        cfg.rules.push(Rule::Iterative { src: 8, vars: [42, 8] });
        cfg.rules.push(Rule::Extended {
            src: 11,
            vars: vec![42, 11, 31],
        });

        cfg.transform_cnf();

        let matches = words
            .into_par_iter()
            .filter(|word| cfg.cyk_algorithm(word).is_some())
            .count();

        matches.to_result()
    }
}
