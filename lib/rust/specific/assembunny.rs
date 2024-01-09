use std::collections::HashMap;

use itertools::Itertools;

use crate::types::ProblemInput;

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(i32),
    Register(String),
}

impl Value {
    fn parse(s: &str) -> Self {
        match s.trim().parse() {
            Ok(imm) => Value::Immediate(imm),
            Err(_) => Value::Register(s.to_string()),
        }
    }

    fn get(&self, registers: &HashMap<String, i32>) -> i32 {
        match self {
            Value::Immediate(imm) => *imm,
            Value::Register(reg) => *registers.get(reg).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy(Value, String),
    Inc(String),
    Dec(String),
    JumpNotZero(Value, Value),
    Toggle(Value),
    Multiply(Value, Value, String),
    Noop,
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub registers: HashMap<String, i32>,
}

impl Program {
    pub fn from_input(input: ProblemInput) -> Self {
        Program {
            instructions: input
                .lines()
                .into_iter()
                .map(|l| {
                    let (inst, args) = l.split_once(' ').unwrap();
                    match inst {
                        "cpy" => {
                            let (src, trg) = args.split_once(' ').unwrap();
                            let value = Value::parse(src);
                            Instruction::Copy(value, trg.trim().to_string())
                        }
                        "inc" => Instruction::Inc(args.to_string()),
                        "dec" => Instruction::Dec(args.to_string()),
                        "jnz" => {
                            let (cond, offset) = args.split_once(' ').unwrap();
                            Instruction::JumpNotZero(Value::parse(cond), Value::parse(offset))
                        }
                        "tgl" => Instruction::Toggle(Value::parse(args)),
                        _ => panic!("Unknown instruction"),
                    }
                })
                .collect_vec(),
            registers: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0usize;
        while pc < self.instructions.len() {
            // println!("{:?}", &self.instructions[pc]);
            match &self.instructions[pc] {
                Instruction::Copy(src, trg) => {
                    *self.registers.get_mut(trg).unwrap() = src.get(&self.registers);
                }
                Instruction::Inc(reg) => *self.registers.get_mut(reg).unwrap() += 1,
                Instruction::Dec(reg) => *self.registers.get_mut(reg).unwrap() -= 1,
                Instruction::JumpNotZero(cond, offset) => {
                    if cond.get(&self.registers) != 0 {
                        pc = (pc as i32 + offset.get(&self.registers) - 1) as usize
                    }
                }

                Instruction::Toggle(off) => {
                    let offset = off.get(&self.registers);
                    if let Some(inst) = self.instructions.get_mut((pc as i32 + offset) as usize) {
                        *inst = match inst {
                            Instruction::Copy(src, trg) => {
                                Instruction::JumpNotZero(src.clone(), Value::Register(trg.clone()))
                            }
                            Instruction::Inc(r) => Instruction::Dec(r.clone()),
                            Instruction::Dec(r) => Instruction::Inc(r.clone()),
                            Instruction::JumpNotZero(cond, off) => match off {
                                Value::Immediate(_) => Instruction::Noop,
                                Value::Register(reg) => Instruction::Copy(cond.clone(), reg.clone()),
                            },
                            Instruction::Toggle(arg) => match arg {
                                Value::Immediate(_) => Instruction::Noop,
                                Value::Register(reg) => Instruction::Inc(reg.clone()),
                            },
                            Instruction::Multiply(_, _, _) => panic!("Wrong optimization"),
                            Instruction::Noop => panic!("Wrong optimization"),
                        }
                    }
                }
                Instruction::Multiply(v1, v2, trg) => {
                    *self.registers.get_mut(trg).unwrap() = v1.get(&self.registers) * v2.get(&self.registers);
                }
                Instruction::Noop => (),
            }
            pc += 1;
        }
    }

    pub fn init_registers(&mut self, registers: Vec<&str>) {
        self.registers = registers.into_iter().map(|r| (r.to_string(), 0)).collect()
    }
}
