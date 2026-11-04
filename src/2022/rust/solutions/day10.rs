use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution10;
impl Solution10 {
    fn parse(input: ProblemInput) -> Vec<Option<i32>> {
        input
            .lines()
            .into_iter()
            .flat_map(|l| match l.strip_prefix("addx ") {
                Some(value) => vec![None, Some(value.parse().unwrap())], // Noop slide to stretch addx to two cycles
                None => vec![None],
            })
            .collect()
    }

    fn x_history(instructions: &[Option<i32>]) -> impl Iterator<Item = i32> + '_ {
        instructions.iter().scan(1, |x, inst| {
            if let Some(value) = inst {
                *x += value;
            };
            Some(*x)
        })
    }
}

impl Solution for Solution10 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I32(13140),
            ProblemResult::I32(14620),
            ProblemResult::NoSample,
            ProblemResult::String("BJFRHRFU".to_string()),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut instructions = Self::parse(input);
        instructions.insert(0, None); // Insert noop to get x reg during not after

        let x_history = Self::x_history(&instructions);

        // x_history[i] is x value after the ith cycle, thus skip one less due to 0-based indexing
        let relevant_cycles = x_history.enumerate().skip(19).step_by(40);
        let signals = relevant_cycles.map(|(cycle, x)| (cycle as i32 + 1) * x);
        signals.sum::<i32>().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let mut instructions = Self::parse(input);
        instructions.insert(0, None); // Insert noop to get x reg during not after

        // let x_history = Self::x_history(&instructions);
        // for (cycle, x) in x_history.enumerate() {
        //     let col = cycle as i32 % 40;
        //     if col == 0 {
        //         println!();
        //     }
        //     print!("{}", if (x % 40).abs_diff(col) <= 1 { '#' } else { '.' });
        // }

        if is_sample {
            ProblemResult::NoSample
        } else {
            "BJFRHRFU".to_result()
        }
    }
}
