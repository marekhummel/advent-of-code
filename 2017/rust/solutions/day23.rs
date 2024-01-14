use aoc_lib::math;
use aoc_lib::solution::Solution;
use aoc_lib::specific::duet::{Instruction, Program, StepResult, Value};
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution23;
impl Solution23 {}

impl Solution for Solution23 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        };

        let mut prog = Program::parse(&input, true);
        let mut multiplies = 0;
        loop {
            match prog.step() {
                StepResult::Normal(Instruction::Mul(_, _)) => multiplies += 1,
                StepResult::Normal(_) => (),
                StepResult::Terminated => break,
                _ => unreachable!(),
            }
        }

        // Check below to see that with n = 99, the two nested loops do 97 iterations each,
        // giving 97 * 97 = 9409 multiplications in total
        multiplies.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        };

        // Assuming a != 0, this is the readable version of the code, a (inverted) primality test:
        // "a" changes the input numbers (a == 0 checks only n := 99)
        // "b" is the number to check for primality, "c" is the upper limit
        // "d" and "e" are loop variables, "f" is the flag if prime, "g" is just a temp
        // "h" counts the non-primes / composites
        // ----
        //
        // n := 109900
        // sentinel := 126900
        //
        // primes := 0
        // loop {
        //   is_prime := true
        //
        //   for d in 2..n {
        //     for e in 2..n {
        //       if d * e == n {
        //         is_prime := false
        //       }
        //     }
        //   }
        //
        //   if not is_prime: primes += 1
        //   if n == sentinel: exit
        //
        //   n += 17
        // }

        // Still parse program to find relevant values
        let mut prog = Program::parse(&input, true);
        prog.instructions
            .insert(0, Instruction::Set("a".to_string(), Value::Immediate(1)));

        // Run first instructions to get values for the range (registers b and c)
        for _ in 0..8 {
            prog.step();
        }
        let start = prog.registers["b"] as u128;
        let end = prog.registers["c"] as u128;

        // Check second to last instruction, which should have the increment for b
        let incr_inst = prog.instructions.iter().nth_back(1).unwrap();
        let Instruction::Sub(_, Value::Immediate(increment)) = incr_inst else {
            panic!()
        };

        // Find composites in that range
        (start..=end)
            .step_by(-increment as usize)
            .filter(|n| !math::is_prime(*n))
            .count()
            .into_some()
    }
}
