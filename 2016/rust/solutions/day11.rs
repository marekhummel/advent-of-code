use std::collections::{HashSet, VecDeque};

use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Microchip(String),
    Generator(String),
}

type Floor = Vec<Item>;

#[derive(Clone)]
struct ColumnState {
    floors: [Floor; 4],
    elevator: usize,
}

impl ColumnState {
    // Encoding for lookup - states are equal, if they have the same types on each level, no matter the element
    fn encode(&self) -> (usize, [(usize, usize); 4]) {
        (
            self.elevator,
            self.floors
                .iter()
                .map(|f| {
                    (
                        f.iter().filter(|i| matches!(i, Item::Generator(_))).count(),
                        f.iter().filter(|i| matches!(i, Item::Microchip(_))).count(),
                    )
                })
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

pub struct Solution11;
impl Solution11 {
    fn parse(input: ProblemInput) -> [Floor; 4] {
        let item_rgx = Regex::new(r"(?P<element>\w+)(?:-compatible)? (?P<type>microchip|generator)").unwrap();

        input
            .lines()
            .into_iter()
            .map(|line| {
                item_rgx
                    .captures_iter(&line)
                    .map(|capture| {
                        let element = capture.name("element").unwrap().as_str().to_string();
                        match capture.name("type").unwrap().as_str() {
                            "microchip" => Item::Microchip(element),
                            "generator" => Item::Generator(element),
                            _ => unreachable!(),
                        }
                    })
                    .collect_vec()
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn run(init: ColumnState) -> u32 {
        let mut seen = HashSet::from([(init.encode())]);
        let mut queue = VecDeque::from([(0u32, init)]);

        while let Some((steps, state)) = queue.pop_front() {
            if Self::all_objects_collected(&state.floors) {
                return steps;
            }

            let pairs = state.floors[state.elevator].iter().cloned().combinations(2);
            let singles = state.floors[state.elevator].iter().cloned().combinations(1);
            for carried_items in pairs.chain(singles) {
                for dir in [1, -1] {
                    let new_elevator = (state.elevator as i8 + dir) as usize;
                    if (0..=3).contains(&new_elevator) {
                        let mut new_floors = state.floors.clone();
                        new_floors[state.elevator].retain(|i| !carried_items.contains(i));
                        new_floors[new_elevator].extend(carried_items.iter().cloned().collect_vec());

                        if !Self::any_chips_fried(&new_floors) {
                            let new_state = ColumnState {
                                floors: new_floors.clone(),
                                elevator: new_elevator,
                            };

                            if !seen.contains(&new_state.encode()) {
                                seen.insert(new_state.encode());
                                queue.push_back((steps + 1, new_state));
                            }
                        }
                    }
                }
            }
        }

        unreachable!()
    }

    fn all_objects_collected(floors: &[Floor]) -> bool {
        floors[0..3].iter().all(|f| f.is_empty())
    }

    fn any_chips_fried(floors: &[Floor]) -> bool {
        for f in floors {
            if f.iter().filter(|i| matches!(i, Item::Generator(_))).count() > 0 {
                for i in f {
                    if let Item::Microchip(element) = i {
                        if !f.contains(&Item::Generator(element.clone())) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

impl Solution for Solution11 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let floors = Self::parse(input);

        let min_steps = Self::run(ColumnState { floors, elevator: 0 });
        min_steps.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let mut floors = Self::parse(input);
        floors[0].extend([
            Item::Generator(String::from("elerium")),
            Item::Microchip(String::from("elerium")),
            Item::Generator(String::from("dilithium")),
            Item::Microchip(String::from("dilithium")),
        ]);

        let min_steps = Self::run(ColumnState { floors, elevator: 0 });
        min_steps.to_result()
    }
}
