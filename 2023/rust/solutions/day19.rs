use std::collections::HashMap;
use std::fmt::Debug;

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

#[derive(Debug)]
struct RulePredicate {
    field: String,
    operator: char,
    threshold: u32,
}

#[derive(Debug)]
struct Rule {
    predicate: Option<RulePredicate>,
    action: String,
}

// Parsed rule with closure to check if rule applies
struct RuleExec<F: Fn(&Part<u32>) -> bool> {
    predicate: Option<F>,
    action: String,
}

#[derive(Debug, Clone)]
struct Part<T: Clone> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T: Clone> Part<T> {
    fn get(&self, field: &str) -> &T {
        match field {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => unreachable!(),
        }
    }

    fn get_mut(&mut self, field: &str) -> &mut T {
        match field {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn size(&self) -> u32 {
        self.to - self.from + 1
    }
}

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> (Vec<Part<u32>>, HashMap<String, Vec<Rule>>) {
        let mut line_iter = input.lines().into_iter();

        let mut workflows = HashMap::new();
        for line in line_iter.by_ref() {
            if line.is_empty() {
                break;
            }

            let (name, rules) = line.trim_end_matches('}').split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(|rule| {
                    if let Some((pred_str, action)) = rule.split_once(':') {
                        let (field, threshold_str) = pred_str.split_once(['<', '>']).unwrap();
                        Rule {
                            predicate: Some(RulePredicate {
                                field: field.to_string(),
                                operator: pred_str.chars().nth(1).unwrap(),
                                threshold: threshold_str.parse::<u32>().unwrap(),
                            }),
                            action: action.to_string(),
                        }
                    } else {
                        Rule {
                            predicate: None,
                            action: rule.to_string(),
                        }
                    }
                })
                .collect_vec();
            workflows.insert(name.to_string(), rules);
        }

        let mut parts = Vec::new();
        for line in line_iter {
            let fields = line.trim_start_matches('{').trim_end_matches('}').split(',');
            let field_map: HashMap<_, _> = fields
                .into_iter()
                .map(|f| f.split_once('=').unwrap())
                .map(|(id, val)| (id, val.parse::<u32>().unwrap()))
                .collect();
            parts.push(Part {
                x: field_map["x"],
                m: field_map["m"],
                a: field_map["a"],
                s: field_map["s"],
            })
        }

        (parts, workflows)
    }

    fn rule_to_executable(rule: Rule) -> RuleExec<impl Fn(&Part<u32>) -> bool> {
        match rule.predicate {
            Some(pred) => {
                let predicate = move |part: &Part<u32>| {
                    let value = part.get(&pred.field);
                    match pred.operator {
                        '<' => *value < pred.threshold,
                        '>' => *value > pred.threshold,
                        _ => unreachable!(),
                    }
                };

                RuleExec {
                    predicate: Some(predicate),
                    action: rule.action.clone(),
                }
            }
            None => RuleExec {
                predicate: None,
                action: rule.action.clone(),
            },
        }
    }

    fn check_part(part: &Part<u32>, workflows: &HashMap<String, Vec<RuleExec<impl Fn(&Part<u32>) -> bool>>>) -> bool {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let mut action = None;
            for rule in workflow.iter() {
                match &rule.predicate {
                    Some(f) if f(part) => action = Some(rule.action.to_string()),
                    None => action = Some(rule.action.to_string()),
                    _ => (),
                }

                if action.is_some() {
                    break;
                }
            }

            match action.unwrap().as_str() {
                "A" => return true,
                "R" => return false,
                next => workflow = workflows.get(next).unwrap(),
            }
        }
    }

    fn update_part(part: &Part<Range>, field: &str, from: u32, to: u32) -> Option<Part<Range>> {
        if from >= to {
            return None;
        }

        let mut new_part = part.clone();
        new_part.get_mut(field).from = from;
        new_part.get_mut(field).to = to;
        Some(new_part)
    }
}

impl Solution for Solution19 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (parts, workflows) = Self::parse(input);
        let exec_workflows: HashMap<_, _> = workflows
            .into_iter()
            .map(|(name, rules)| (name, rules.into_iter().map(Self::rule_to_executable).collect_vec()))
            .collect();

        parts
            .into_iter()
            .filter(|part| Self::check_part(part, &exec_workflows))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum::<u32>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (_, workflows) = Self::parse(input);

        let initial_part = Part {
            x: Range { from: 1, to: 4000 },
            m: Range { from: 1, to: 4000 },
            a: Range { from: 1, to: 4000 },
            s: Range { from: 1, to: 4000 },
        };

        let mut completed_ranges = Vec::new();
        let mut ranged_parts = Vec::from([(initial_part, "in".to_owned(), 0)]);

        while let Some((part, workflow_name, rule_index)) = ranged_parts.pop() {
            // Sentinels
            match workflow_name.as_str() {
                "A" => {
                    completed_ranges.push(part.clone());
                    continue;
                }
                "R" => continue,
                _ => (),
            }

            let workflow = workflows.get(&workflow_name).unwrap();
            let rule = &workflow[rule_index];

            match &rule.predicate {
                None => ranged_parts.push((part.clone(), rule.action.clone(), 0)),
                Some(pred) => {
                    let curr = part.get(&pred.field);
                    match pred.operator {
                        '<' => {
                            // Positive
                            if let Some(new) = Self::update_part(&part, &pred.field, curr.from, pred.threshold - 1) {
                                ranged_parts.push((new, rule.action.clone(), 0));
                            }

                            // Negative
                            if let Some(new) = Self::update_part(&part, &pred.field, pred.threshold, curr.to) {
                                ranged_parts.push((new, workflow_name, rule_index + 1));
                            }
                        }
                        '>' => {
                            // Positive
                            if let Some(new) = Self::update_part(&part, &pred.field, pred.threshold + 1, curr.to) {
                                ranged_parts.push((new, rule.action.clone(), 0));
                            }

                            // Negative
                            if let Some(new) = Self::update_part(&part, &pred.field, curr.from, pred.threshold) {
                                ranged_parts.push((new, workflow_name, rule_index + 1));
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        completed_ranges
            .into_iter()
            .map(|r| r.x.size() as u128 * r.m.size() as u128 * r.a.size() as u128 * r.s.size() as u128)
            .sum::<u128>()
            .into_some()
    }
}
