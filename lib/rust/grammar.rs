use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
};

use itertools::Itertools;

#[derive(Clone)]
pub enum Rule<NT, T>
where
    NT: Clone + PartialEq + Eq,
    T: Clone + PartialEq + Eq,
{
    Unit { src: NT, terminal: T },
    Iterative { src: NT, vars: [NT; 2] },
    Extended { src: NT, vars: Vec<NT> }, // Used only prior to cnf
}

impl<NT, T> Rule<NT, T>
where
    NT: Clone + PartialEq + Eq,
    T: Clone + PartialEq + Eq,
{
    pub fn source(&self) -> &NT {
        match self {
            Rule::Unit {
                src: source,
                terminal: _,
            } => source,
            Rule::Iterative { src: source, vars: _ } => source,
            Rule::Extended { src: source, vars: _ } => source,
        }
    }

    pub fn nonterminals(&self) -> Vec<NT> {
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

impl<NT, T> Display for Rule<NT, T>
where
    NT: Clone + Debug + Display + PartialEq + Eq,
    T: Clone + Display + PartialEq + Eq,
{
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

pub struct CFG<NT, T>
where
    NT: Clone + PartialEq + Eq,
    T: Clone + PartialEq + Eq,
{
    pub rules: Vec<Rule<NT, T>>,
    pub start: NT,
    /// Function to generate new NTs in CNF conversion
    pub sub_func: fn(usize) -> NT,
}

impl<NT, T> CFG<NT, T>
where
    NT: Clone + PartialEq + Eq + Hash + Debug,
    T: Clone + PartialEq + Eq,
{
    pub fn cyk_algorithm(&self, word: &[T]) -> Option<Vec<Rule<NT, T>>> {
        let n = word.len();
        let mut v = vec![vec![vec![]; n]; n];
        let mut trace = vec![vec![vec![]; n]; n];

        // Apply unit rules (length 1 rules)
        for (i, wi) in word.iter().enumerate() {
            for rule in &self.rules {
                if let Rule::Unit { src: source, terminal } = rule {
                    if terminal == wi {
                        v[i].get_mut(0).unwrap().push(source);
                        trace[i][0].push((rule, (0, 0, 0), (0, 0, 0)));
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
                    for rule in &self.rules {
                        if let Rule::Iterative { src, vars } = rule {
                            let [y, z] = vars;
                            if let Some(py) = v[s1][p1 - 1].iter().position(|nt| nt == &y) {
                                if let Some(pz) = v[s2][p2 - 1].iter().position(|nt| nt == &z) {
                                    v[s1].get_mut(length - 1).unwrap().push(src);
                                    trace[s1]
                                        .get_mut(length - 1)
                                        .unwrap()
                                        .push((rule, (s1, p1, py), (s2, p2, pz)));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Match if start is in top left cell
        if !v[0][n - 1].contains(&&self.start) {
            return None;
        }

        // Retrace rules
        let mut production_tree = Vec::new();
        let mut tree = vec![trace[0][n - 1].clone()[0]]; // Choose rule with S
        while let Some((rule, (s1, p1, pos1), (s2, p2, pos2))) = tree.pop() {
            production_tree.push(rule.clone());
            if !matches!(rule, Rule::Unit { src: _, terminal: _ }) {
                tree.insert(0, *trace[s1][p1 - 1].get(pos1).unwrap());
                tree.insert(1, *trace[s2][p2 - 1].get(pos2).unwrap());
            }
        }

        Some(production_tree)
    }

    pub fn transform_cnf(&mut self) {
        // Find extended rules and create substitute rules to meet CNF
        let mut shorten: VecDeque<_> = self.rules.iter().filter(|p| p.is_extended()).cloned().collect();
        self.rules.retain(|p| !p.is_extended());

        let mut substitutes: HashMap<(NT, NT), NT> = HashMap::new();
        while let Some(Rule::Extended { src, vars }) = shorten.pop_front() {
            // If vars is just a single NT, it's a chain rule
            if vars.len() == 1 {
                let mut new_rules = Vec::new();
                for rule in self.rules.iter() {
                    if rule.source() == &vars[0] {
                        let new_rule = match rule {
                            Rule::Unit { src: _, terminal } => Rule::Unit {
                                src: src.clone(),
                                terminal: terminal.clone(),
                            },
                            Rule::Iterative { src: _, vars } => Rule::Iterative {
                                src: src.clone(),
                                vars: vars.clone(),
                            },
                            Rule::Extended { src: _, vars } => Rule::Extended {
                                src: src.clone(),
                                vars: vars.clone(),
                            },
                        };

                        new_rules.push(new_rule);
                    }
                }

                self.rules.extend(new_rules);
                continue;
            }

            // Create substitute for every two of the non terminals
            let mut subs = Vec::new();
            for chunk in vars.iter().chunks(2).into_iter() {
                if let Some((nt1, nt2)) = chunk.collect_tuple() {
                    if let Some(sub) = substitutes.get(&(nt1.clone(), nt2.clone())) {
                        subs.push(sub.clone());
                    } else {
                        let new_sub = (self.sub_func)(substitutes.len());
                        substitutes.insert((nt1.clone(), nt2.clone()), new_sub.clone());
                        subs.push(new_sub.clone());
                    }
                }
            }

            match subs.len() {
                2 => {
                    // If only two subs used, we have a new final rule
                    self.rules.push(Rule::Iterative {
                        src,
                        vars: subs.try_into().unwrap(),
                    });
                }
                1 => {
                    // If only one sub left, combine it with the left over non terminal
                    if vars.len() & 1 == 1 {
                        self.rules.push(Rule::Iterative {
                            src,
                            vars: [subs.pop().unwrap(), vars.last().unwrap().clone()],
                        });
                    }
                }
                _ => {
                    // Too many subs left, repeat process on this new rule
                    shorten.push_back(Rule::Extended { src, vars: subs });
                }
            }
        }

        // Append all substitutions
        for ((nt1, nt2), source) in substitutes {
            self.rules.push(Rule::Iterative {
                src: source,
                vars: [nt1, nt2],
            });
        }
    }
}
