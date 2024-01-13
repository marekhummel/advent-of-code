use std::collections::{HashMap, VecDeque};

use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::Itertools;

type RegValue = i64;

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(RegValue),
    Register(String),
}

impl Value {
    fn parse(s: &str) -> Self {
        match s.trim().parse() {
            Ok(imm) => Value::Immediate(imm),
            Err(_) => Value::Register(s.to_string()),
        }
    }

    fn get(&self, registers: &HashMap<String, RegValue>) -> RegValue {
        match self {
            Value::Immediate(imm) => *imm,
            Value::Register(reg) => *registers.get(reg).unwrap_or(&0),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Sound(Value),
    Recover(Value),
    Send(Value),
    Receive(String),
    Set(String, Value),
    Add(String, Value),
    Mul(String, Value),
    Mod(String, Value),
    Jump(Value, Value),
}

#[derive(PartialEq, Eq)]
enum StepResult {
    Normal,
    Send(RegValue),
    Receiving,
    Recover(RegValue),
}

struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<String, RegValue>,
    pc: usize,
    queue: VecDeque<RegValue>,
}

impl Program {
    fn parse(input: &ProblemInput, misread: bool) -> Self {
        Program {
            instructions: input
                .lines()
                .into_iter()
                .map(|l| {
                    let (inst, args) = l.split_once(' ').unwrap();
                    match inst {
                        "set" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let y_val = Value::parse(y);
                            Instruction::Set(x.trim().to_string(), y_val)
                        }
                        "add" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let y_val = Value::parse(y);
                            Instruction::Add(x.trim().to_string(), y_val)
                        }
                        "mul" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let y_val = Value::parse(y);
                            Instruction::Mul(x.trim().to_string(), y_val)
                        }
                        "mod" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let y_val = Value::parse(y);
                            Instruction::Mod(x.trim().to_string(), y_val)
                        }
                        "snd" if misread => Instruction::Sound(Value::parse(args)),
                        "rcv" if misread => Instruction::Recover(Value::parse(args)),
                        "snd" => Instruction::Send(Value::parse(args)),
                        "rcv" => Instruction::Receive(args.trim().to_string()),
                        "jgz" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let x_val = Value::parse(x);
                            let y_val = Value::parse(y);
                            Instruction::Jump(x_val, y_val)
                        }
                        _ => panic!("Unknown instruction"),
                    }
                })
                .collect_vec(),
            registers: HashMap::new(),
            pc: 0,
            queue: VecDeque::new(),
        }
    }

    fn step(&mut self) -> StepResult {
        let inst = &self.instructions[self.pc];
        self.pc += 1;
        match inst {
            Instruction::Sound(x) => self.queue.push_back(x.get(&self.registers)),
            Instruction::Recover(x) => {
                if x.get(&self.registers) != 0 {
                    return StepResult::Recover(*self.queue.back().unwrap());
                }
            }
            Instruction::Send(x) => return StepResult::Send(x.get(&self.registers)),
            Instruction::Receive(x) => {
                if let Some(recv) = self.queue.pop_front() {
                    *self.registers.entry(x.to_string()).or_insert(0) = recv;
                } else {
                    self.pc -= 1;
                    return StepResult::Receiving;
                }
            }
            Instruction::Set(x, y) => *self.registers.entry(x.to_string()).or_insert(0) = y.get(&self.registers),
            Instruction::Add(x, y) => *self.registers.entry(x.to_string()).or_insert(0) += y.get(&self.registers),
            Instruction::Mul(x, y) => *self.registers.entry(x.to_string()).or_insert(0) *= y.get(&self.registers),
            Instruction::Mod(x, y) => *self.registers.entry(x.to_string()).or_insert(0) %= y.get(&self.registers),
            Instruction::Jump(x, y) => {
                if x.get(&self.registers) > 0 {
                    self.pc = (self.pc as RegValue + y.get(&self.registers) - 1) as usize
                }
            }
        }

        StepResult::Normal
    }
}

pub struct Solution18;
impl Solution18 {}

impl Solution for Solution18 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut prog = Program::parse(&input, true);

        loop {
            if let StepResult::Recover(x) = prog.step() {
                return x.into_some();
            }
        }
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let mut p0 = Program::parse(&input, false);
        let mut p1 = Program::parse(&input, false);

        p0.instructions
            .insert(0, Instruction::Set("p".to_string(), Value::Immediate(0)));
        p1.instructions
            .insert(0, Instruction::Set("p".to_string(), Value::Immediate(1)));

        let mut p1_counter = 0;
        loop {
            // Move
            let r0 = p0.step();
            let r1 = p1.step();

            // Deadlock, both receive
            if r0 == StepResult::Receiving && r1 == StepResult::Receiving {
                break;
            }

            // Update queue of p1, if p0 sent sth
            if let StepResult::Send(v0) = r0 {
                p1.queue.push_back(v0);
            }

            // Update queue of p0, if p1 sent sth
            if let StepResult::Send(v1) = r1 {
                p0.queue.push_back(v1);
                p1_counter += 1;
            }
        }

        p1_counter.into_some()
    }
}
