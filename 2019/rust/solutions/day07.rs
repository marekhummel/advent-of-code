use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use aoc_lib::specific::intcode::Program;
use itertools::Itertools;

pub struct Solution07;
impl Solution07 {}

impl Solution for Solution07 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I128(65210),
            ProblemResult::I128(13848),
            ProblemResult::I128(139629729),
            ProblemResult::I128(12932154),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut thruster_signal = i128::MIN;
        for phase_settings in (0..5).permutations(5) {
            let mut output = 0;
            for phase in phase_settings {
                let mut amp_program = Program::init(&input.string());
                amp_program.input.push_back(phase);
                amp_program.input.push_back(output);

                amp_program.execute();
                output = amp_program.output[0];
            }

            thruster_signal = thruster_signal.max(output)
        }

        thruster_signal.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut thruster_signal = i128::MIN;
        for phase_settings in (5..10).permutations(5) {
            // Init amplifiers
            let mut amplifiers = (0..5).map(|_| Program::init(&input.string())).collect_vec();
            for (amp, phase) in amplifiers.iter_mut().zip_eq(phase_settings.iter()) {
                amp.input.push_back(*phase);
            }

            // Run feedback loop
            let mut output = 0;
            for current in (0..5).cycle() {
                // Forward signal
                let amp_program = amplifiers.get_mut(current).unwrap();
                amp_program.input.push_back(output);

                match amp_program.execute_until_output() {
                    Some(signal) => output = signal,
                    None => break, // Exit if halted
                }
            }

            // Check last output from amplifier E
            let final_output = amplifiers[4].output.last().unwrap();
            thruster_signal = thruster_signal.max(*final_output)
        }

        thruster_signal.to_result()
    }
}
