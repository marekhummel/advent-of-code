use std::collections::HashMap;

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;
use itertools::Itertools;

pub struct Solution11;
impl Solution11 {
    fn run_robot(mut brain: Program, panels: &mut HashMap<Position, i128>) {
        let mut pos = Position::zero();
        let mut dir = Direction::North;

        loop {
            let color = *panels.get(&pos).unwrap_or(&0);
            brain.input.push_back(color);
            let Some(paint) = brain.execute_until_output() else {
                break;
            };
            let Some(turn) = brain.execute_until_output() else {
                break;
            };

            panels.insert(pos, paint);
            dir = match turn {
                0 => dir.left(),
                1 => dir.right(),
                _ => unreachable!(),
            };

            pos = pos.advance_by(dir, 1);
        }
    }

    #[allow(dead_code)]
    fn print_panels(panels: &HashMap<Position, i128>) {
        let Some((min_x, max_x)) = panels.keys().map(|pos| pos.x).minmax().into_option() else {
            panic!()
        };
        let Some((min_y, max_y)) = panels.keys().map(|pos| pos.y).minmax().into_option() else {
            panic!()
        };

        println!();
        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                print!(
                    "{}",
                    if *panels.get(&Position { x, y }).unwrap_or(&0) == 1 {
                        '#'
                    } else {
                        ' '
                    }
                )
            }
            println!();
        }
    }
}

impl Solution for Solution11 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let brain = Program::init(&input.string());
        let mut panels = HashMap::new();
        Self::run_robot(brain, &mut panels);

        panels.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        if is_sample {
            return ProblemResult::NoSample;
        }

        let brain = Program::init(&input.string());
        let mut panels = HashMap::from([(Position::zero(), 1)]);
        Self::run_robot(brain, &mut panels);

        // --- Print to see message
        // Self::print_panels(&panels);
        "PGUEPLPR".to_result()
    }
}
