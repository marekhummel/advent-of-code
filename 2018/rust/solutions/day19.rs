use aoc_lib::solution::Solution;
use aoc_lib::specific::wristdevice::{Instruction, Op};
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> (Vec<Instruction>, usize) {
        let lines = input.lines();

        let pc_link = lines[0].strip_prefix("#ip ").unwrap().parse().unwrap();

        let instructions = lines[1..]
            .iter()
            .map(|l| {
                let (op_str, a, b, c) = l.split_whitespace().collect_tuple().unwrap();
                let opcode = match op_str {
                    "addi" => Op::Addi,
                    "addr" => Op::Addr,
                    "muli" => Op::Muli,
                    "mulr" => Op::Mulr,
                    "bani" => Op::Bani,
                    "banr" => Op::Banr,
                    "bori" => Op::Bori,
                    "borr" => Op::Borr,
                    "seti" => Op::Seti,
                    "setr" => Op::Setr,
                    "gtir" => Op::Gtir,
                    "gtri" => Op::Gtri,
                    "gtrr" => Op::Gtrr,
                    "eqir" => Op::Eqir,
                    "eqri" => Op::Eqri,
                    "eqrr" => Op::Eqrr,
                    _ => unreachable!(),
                };

                Instruction {
                    opcode,
                    a: a.parse().unwrap(),
                    b: b.parse().unwrap(),
                    c: c.parse().unwrap(),
                }
            })
            .collect();

        (instructions, pc_link)
    }
}

impl Solution for Solution19 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (program, pc_link) = Self::parse(input);

        let mut registers = vec![0; 6];
        let mut pc = 0;
        while pc < program.len() {
            registers[pc_link] = pc;
            program[pc].execute(&mut registers);
            pc = registers[pc_link];
            pc += 1;
        }

        registers[0].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let (program, pc_link) = Self::parse(input);

        // Simplified program, computes divsum of integer:
        // n is stored in reg #6, temp values are in #3, divisors in #2,
        // their possible inverse in #5, the IP in #4 and the final divsum in #1.
        // Note that reg #1 is initially used to change n.
        // Simplification possible by treating all instructions with target #4 as jumps,
        // and all instructions that use #4 can be replaced by immediates with their instruction number.

        // n = 2 * 2 * 19 * 11 + 5 * 22 + 21
        // if divsum > 0 {
        //     n = n + (27 * 28 + 29) * 30 * 14 * 32
        //     divsum = 0
        // }
        //
        // for div in 1..=n {
        //     for inv in 1..=n {
        //         if inv * div == n {
        //             divsum += div
        //         }
        //     }
        // }

        // Run until we are back at the top again, meaning n is computed.
        let mut registers = vec![0; 6];
        registers[0] = 1;
        let mut pc = 0;
        while pc < program.len() {
            registers[pc_link] = pc;
            program[pc].execute(&mut registers);
            pc = registers[pc_link];
            pc += 1;

            if pc == 1 {
                break;
            }
        }

        // Compute divsum
        let n = registers[5] as u32;
        let root = (n as f64).sqrt() as u32;
        let divsum: u32 = (1..=root).filter(|f| n % f == 0).map(|f| f + n / f).sum();
        divsum.into_some()
    }
}
