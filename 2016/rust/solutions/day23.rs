use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

use aoc_lib::specific::assembunny::{Instruction, Program, Value};

pub struct Solution23;
impl Solution23 {
    // Finds nested for loops which implement multiplication by repeated addition
    fn optimize_multiplication(program: &mut Program) {
        // cpy a d
        // cpy 0 a
        // cpy b c
        // inc a
        // dec c
        // jnz c -2
        // dec d
        // jnz d -5
        let mut replacements = Vec::new();
        for (i, slice) in program.instructions[..].windows(8).enumerate() {
            if let [Instruction::Copy(Value::Register(s1), t1), Instruction::Copy(Value::Immediate(0), t2), Instruction::Copy(Value::Register(s3), t3), Instruction::Inc(s4), Instruction::Dec(s5), Instruction::JumpNotZero(Value::Register(s6), Value::Immediate(-2)), Instruction::Dec(s7), Instruction::JumpNotZero(Value::Register(s8), Value::Immediate(-5))] =
                slice
            {
                if [s1, t2, s4].iter().all_equal() && [t3, s5, s6].iter().all_equal() && [t1, s7, s8].iter().all_equal()
                {
                    replacements.push((
                        i,
                        [
                            vec![Instruction::Noop; 5],
                            vec![
                                Instruction::Multiply(
                                    Value::Register(s1.clone()),
                                    Value::Register(s3.clone()),
                                    s1.clone(),
                                ),
                                Instruction::Copy(Value::Immediate(0), t3.clone()),
                                Instruction::Copy(Value::Immediate(0), t1.clone()),
                            ],
                        ]
                        .concat(),
                    ));
                }
            }
        }

        for (i, repl) in replacements {
            program.instructions.splice(i..i + 8, repl);
        }
    }
}

impl Solution for Solution23 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(7), "a".to_string()));

        program.init_registers(vec!["a", "b", "c", "d"]);
        program.run();
        program.registers["a"].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut program = Program::from_input(input);
        program
            .instructions
            .insert(0, Instruction::Copy(Value::Immediate(12), "a".to_string()));

        Self::optimize_multiplication(&mut program);
        program.init_registers(vec!["a", "b", "c", "d"]);
        program.run();
        program.registers["a"].into_some()
    }
}
