use aoc_lib::iterator::ParsedExt;
use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution07;
impl Solution07 {}

impl Solution for Solution07 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let intcode = input.string().split(',').parsed::<i32>().collect_vec();

        let mut thruster_signal = i32::MIN;
        for phase_settings in (0..5).permutations(5) {
            let mut output = 0;
            for phase in phase_settings {
                let mut amp_program = Program::init(intcode.clone());
                amp_program.input.push_back(phase);
                amp_program.input.push_back(output);

                amp_program.execute();
                output = amp_program.output[0];
            }

            thruster_signal = thruster_signal.max(output)
        }

        thruster_signal.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let intcode = input.string().split(',').parsed::<i32>().collect_vec();

        let mut thruster_signal = i32::MIN;
        for phase_settings in (5..10).permutations(5) {
            // Init amplifiers
            let mut amplifiers = (0..5).map(|_| Program::init(intcode.clone())).collect_vec();
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

        thruster_signal.into_some()
    }
}
