use std::collections::{HashMap, HashSet};

use aoc_lib::math::lcm;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Untyped,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug)]
struct Module {
    name: String,
    mtype: ModuleType,
    targets: HashSet<String>,
}

impl Module {
    fn receive_pulse(&mut self, from: &str, pulse: bool) -> Vec<Pulse> {
        let out_pulse = match self.mtype {
            ModuleType::Untyped => Some(pulse),
            ModuleType::FlipFlop(ref mut state) => match pulse {
                true => None,
                false => {
                    *state = !*state;
                    Some(*state)
                }
            },
            ModuleType::Conjunction(ref mut states) => {
                *states.get_mut(from).unwrap() = pulse;
                let send = !states.values().all(|v| *v);
                Some(send)
            }
        };

        out_pulse.map_or(Vec::new(), |out| {
            self.targets
                .iter()
                .map(|t| (self.name.clone(), t.clone(), out))
                .collect_vec()
        })
    }
}

type Pulse = (String, String, bool);

pub struct Solution20;
impl Solution20 {
    fn parse(input: ProblemInput) -> HashMap<String, Module> {
        let mut modules: HashMap<_, _> = input
            .lines()
            .into_iter()
            .map(|line| {
                let (source, targets_str) = line.split_once("->").unwrap();
                let (mtype, name) = match source.chars().next().unwrap() {
                    '%' => (ModuleType::FlipFlop(false), &source[1..]),
                    '&' => (ModuleType::Conjunction(HashMap::new()), &source[1..]),
                    _ => (ModuleType::Untyped, source),
                };
                let targets = targets_str.split(',').map(|t| t.trim().to_string()).collect();
                let module = Module {
                    name: name.trim().to_string(),
                    mtype,
                    targets,
                };

                (module.name.clone(), module)
            })
            .collect();

        let module_names = modules.keys().cloned().collect_vec();
        for m_name in module_names {
            let m_targets = modules.get(&m_name).unwrap().targets.clone();
            for tm_name in m_targets {
                if let Some(tm) = modules.get_mut(&tm_name.clone()) {
                    if let ModuleType::Conjunction(ref mut states) = tm.mtype {
                        states.insert(m_name.clone(), false);
                    }
                }
            }
        }

        modules
    }

    fn press_button(modules: &mut HashMap<String, Module>) -> Vec<Pulse> {
        let mut all_pulses = Vec::new();

        let mut pulses = Vec::from([("button".to_string(), "broadcaster".to_string(), false)]);
        while !&pulses.is_empty() {
            all_pulses.extend(pulses.clone());
            let mut new_pulses = Vec::new();
            for (sender, receiver, pulse) in pulses.into_iter() {
                if let Some(recv_mod) = modules.get_mut(&receiver) {
                    let responses = recv_mod.receive_pulse(&sender, pulse);
                    new_pulses.extend(responses.into_iter());
                }
            }
            pulses = new_pulses;
        }

        all_pulses
    }

    fn compute_state(modules: &HashMap<String, Module>) -> HashMap<String, ModuleType> {
        modules.values().map(|m| (m.name.clone(), m.mtype.clone())).collect()
    }
}

impl Solution for Solution20 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        const BUTTON_PRESSES: usize = 1000;
        let mut modules = Self::parse(input);
        let mut states = Vec::new();
        let mut pulse_counts = Vec::new();

        for _ in 0..BUTTON_PRESSES {
            let state = Self::compute_state(&modules);
            if states.contains(&state) {
                break;
            }
            states.push(state);

            let pulses = Self::press_button(&mut modules);
            let (highs, lows): (Vec<_>, Vec<_>) = pulses.into_iter().partition(|(_, _, pulse)| *pulse);
            pulse_counts.push((lows.len(), highs.len()));
        }

        let (lows, highs) = pulse_counts
            .iter()
            .cycle()
            .take(BUTTON_PRESSES)
            .fold((0, 0), |(acc_l, acc_h), (l, h)| (acc_l + l, acc_h + h));
        (lows * highs).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut modules = Self::parse(input);

        // Looking at the input, we know rx is only the target of one conjuction: "cs"
        const FINAL: &str = "cs";

        // Not defined for sample
        if is_sample {
            return ProblemResult::NoSample;
        }

        if let ModuleType::Conjunction(states) = &modules[FINAL].mtype {
            let inputs: HashSet<String> = states.keys().cloned().collect();
            let mut first_high: HashMap<String, u64> = HashMap::new();

            // Run simulation
            for i in 1u64.. {
                let pulses = Self::press_button(&mut modules);
                for (from, to, pulse) in pulses {
                    // Catch all pulses sent to "cs" and store the iteration of the first high pulse per input
                    if pulse && to == FINAL && inputs.contains(&from) && !first_high.contains_key(&from) {
                        first_high.insert(from, i);
                    }
                }

                if first_high.len() == inputs.len() {
                    break;
                }
            }

            // Once all inputs have sent a high once, the first iteration on which all inputs send high
            // and thus make "cs" send a high to "rx" is the lcm of all those stored values.
            return (lcm(&first_high.values().cloned().collect_vec())).to_result();
        };

        unreachable!()
    }
}
