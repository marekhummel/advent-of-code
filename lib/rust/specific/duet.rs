use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::prelude::types::ProblemInput;

type RegValue = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Sound(Value),       // Sound(X) -> Store X
    Recover(Value),     // Recover(X) -> Recover stored sound if X != 0
    Send(Value),        // Send(X) -> Send value in X
    Receive(String),    // Receive(X) -> Take value from queue and save in X
    Set(String, Value), // Set(X, Y) -> X := Y
    Add(String, Value), // Add(X, Y) -> X := X + Y
    Sub(String, Value), // Sub(X, Y) -> X := X - Y
    Mul(String, Value), // Mul(X, Y) -> X := X * Y
    Mod(String, Value), // Mod(X, Y) -> X := X % Y
    Jgz(Value, Value),  // Jgz(X, Y) -> Jump by Y if X > 0
    Jnz(Value, Value),  // Jnz(X, Y) -> Jump by Y if X != 0
}

#[derive(PartialEq, Eq)]
pub enum StepResult {
    Normal(Instruction),
    Send(RegValue),
    Receiving,
    Recover(RegValue),
    Terminated,
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub registers: HashMap<String, RegValue>,
    pub queue: VecDeque<RegValue>,
    pc: usize,
}

impl Program {
    pub fn parse(input: &ProblemInput, misread: bool) -> Self {
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
                        "sub" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let y_val = Value::parse(y);
                            Instruction::Sub(x.trim().to_string(), y_val)
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
                            Instruction::Jgz(x_val, y_val)
                        }
                        "jnz" => {
                            let (x, y) = args.split_once(' ').unwrap();
                            let x_val = Value::parse(x);
                            let y_val = Value::parse(y);
                            Instruction::Jnz(x_val, y_val)
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

    pub fn step(&mut self) -> StepResult {
        if self.pc >= self.instructions.len() {
            return StepResult::Terminated;
        }

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
            Instruction::Sub(x, y) => *self.registers.entry(x.to_string()).or_insert(0) -= y.get(&self.registers),
            Instruction::Mul(x, y) => *self.registers.entry(x.to_string()).or_insert(0) *= y.get(&self.registers),
            Instruction::Mod(x, y) => *self.registers.entry(x.to_string()).or_insert(0) %= y.get(&self.registers),
            Instruction::Jgz(x, y) => {
                if x.get(&self.registers) > 0 {
                    self.pc = (self.pc as RegValue + y.get(&self.registers) - 1) as usize
                }
            }
            Instruction::Jnz(x, y) => {
                if x.get(&self.registers) != 0 {
                    self.pc = (self.pc as RegValue + y.get(&self.registers) - 1) as usize
                }
            }
        }

        StepResult::Normal(inst.clone())
    }
}
