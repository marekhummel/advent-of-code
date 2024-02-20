use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Clone)]
enum Rule {
    Unit { src: String, terminal: String },
    Iterative { src: String, vars: [String; 2] },
    Extended { src: String, vars: Vec<String> }, // used during parsing only
}

impl Rule {
    fn source(&self) -> &str {
        match self {
            Rule::Unit {
                src: source,
                terminal: _,
            } => source,
            Rule::Iterative { src: source, vars: _ } => source,
            Rule::Extended { src: source, vars: _ } => source,
        }
    }

    fn nonterminals(&self) -> Vec<String> {
        match self {
            Rule::Unit { src: _, terminal: _ } => vec![],
            Rule::Iterative {
                src: _,
                vars: non_terminals,
            } => non_terminals.to_vec(),
            Rule::Extended {
                src: _,
                vars: non_terminals,
            } => non_terminals.clone(),
        }
    }

    fn is_extended(&self) -> bool {
        matches!(self, Rule::Extended { src: _, vars: _ })
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Unit { src: source, terminal } => write!(f, "{} -> {}", source, terminal),
            Rule::Iterative {
                src: source,
                vars: non_terminals,
            } => {
                write!(f, "{} -> {} {}", source, non_terminals[0], non_terminals[1])
            }
            Rule::Extended {
                src: source,
                vars: non_terminals,
            } => write!(f, "{} -> {:?}", source, non_terminals),
        }
    }
}

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> (Vec<String>, Vec<Rule>) {
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

    fn transform_cnf(rules: &mut Vec<Rule>) {
        // Find extended rules and create substitute rules to meet CNF
        let mut shorten: VecDeque<_> = rules.iter().filter(|p| p.is_extended()).cloned().collect();
        rules.retain(|p| !p.is_extended());

        let mut substitutes: HashMap<(String, String), String> = HashMap::new();
        while let Some(Rule::Extended { src, vars }) = shorten.pop_front() {
            // Create substitute for every two of the non terminals
            let mut subs = Vec::new();
            for chunk in vars.iter().chunks(2).into_iter() {
                if let Some((nt1, nt2)) = chunk.collect_tuple() {
                    if let Some(sub) = substitutes.get(&(nt1.clone(), nt2.clone())) {
                        subs.push(sub.clone());
                    } else {
                        let new_sub = format!("Sub{}", substitutes.len() + 1);
                        substitutes.insert((nt1.clone(), nt2.clone()), new_sub.clone());
                        subs.push(new_sub.clone());
                    }
                }
            }

            if let [s1, s2] = &subs[..] {
                // If only two subs used, we have a new final rule
                rules.push(Rule::Iterative {
                    src,
                    vars: [s1.clone(), s2.clone()],
                });
            } else if let [s] = &subs[..] {
                // If only one sub left, combine it with the left over non terminal
                if vars.len() & 1 == 1 {
                    rules.push(Rule::Iterative {
                        src,
                        vars: [s.clone(), vars.last().unwrap().clone()],
                    });
                }
            } else {
                // Too many subs left, repeat process on this new rule
                shorten.push_back(Rule::Extended { src, vars: subs });
            }
        }

        // Append all substitutions
        for ((nt1, nt2), source) in substitutes {
            rules.push(Rule::Iterative {
                src: source,
                vars: [nt1, nt2],
            });
        }
    }

    fn add_terminals(rules: &mut Vec<Rule>, start: &str) {
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

    fn cyk_algorithm(rules: &[Rule], word: &[String], start: String) -> Vec<Rule> {
        let n = word.len();
        let v = vec![vec![RefCell::new(vec![]); n]; n];
        let trace = vec![vec![RefCell::new(vec![]); n]; n];

        // Apply unit rules (length 1 rules)
        for (i, wi) in word.iter().enumerate() {
            for rule in rules {
                if let Rule::Unit { src: source, terminal } = rule {
                    if terminal == wi {
                        v[i][0].borrow_mut().push(source);
                        trace[i][0].borrow_mut().push((rule, (0, 0, 0), (0, 0, 0)));
                    }
                }
            }
        }

        // DP to iteratively concatenate two NTs Y and Z to X if X -> YZ is a rule
        for length in 2..=n {
            for s1 in 0..n - length + 1 {
                for p1 in 1..length {
                    let s2 = s1 + p1;
                    let p2 = length - p1;
                    for rule in rules {
                        if let Rule::Iterative { src, vars } = rule {
                            let [y, z] = vars;
                            if let Some(py) = v[s1][p1 - 1].borrow().iter().position(|nt| nt == &y) {
                                if let Some(pz) = v[s2][p2 - 1].borrow().iter().position(|nt| nt == &z) {
                                    v[s1][length - 1].borrow_mut().push(src);
                                    trace[s1][length - 1]
                                        .borrow_mut()
                                        .push((rule, (s1, p1, py), (s2, p2, pz)));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Must've found start symbol in top left node
        assert!(v[0][n - 1].borrow().contains(&&start));

        // Retrace rules
        let mut production_tree = Vec::new();
        let mut tree = vec![trace[0][n - 1].borrow().clone()[0]]; // Choose rule with S
        while let Some((rule, (s1, p1, pos1), (s2, p2, pos2))) = tree.pop() {
            production_tree.push(rule.clone());
            if !matches!(rule, Rule::Unit { src: _, terminal: _ }) {
                tree.insert(0, *trace[s1][p1 - 1].borrow().get(pos1).unwrap());
                tree.insert(1, *trace[s2][p2 - 1].borrow().get(pos2).unwrap());
            }
        }

        production_tree
    }
}

impl Solution for Solution19 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (molecule, rules) = Self::parse(input);

        let mut new_molecules = HashSet::new();
        for rule in rules {
            let src = rule.source();
            let positions = molecule.iter().positions(|elem| *elem == src).collect_vec();
            for pos in positions {
                let new_molecule = [&molecule[0..pos], &rule.nonterminals(), &molecule[pos + 1..]].concat();
                // println!("{new_molecule:?} ({rule} on {pos})");
                new_molecules.insert(new_molecule);
            }
        }

        new_molecules.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (mut word, mut cfg) = Self::parse(input);
        let start = "e";

        // Transform given CFG to CNF for CYK algorithm
        Self::transform_cnf(&mut cfg);
        Self::add_terminals(&mut cfg, start);

        // Transform word to lower case to work with terminals
        word = word.into_iter().map(|elem| elem.to_lowercase()).collect_vec();

        // Sample is one application short due to manual adaptation of language for start symbol
        let rules = Self::cyk_algorithm(&cfg, &word, start.to_string());

        // Count all rules, except for the substitute and terminal ones as they weren't in the original
        rules
            .into_iter()
            .filter(|p| matches!(p, Rule::Iterative { src, vars: _ } if !src.starts_with("Sub")))
            .count()
            .to_result()
    }
}
