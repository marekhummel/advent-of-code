use std::collections::{HashMap, HashSet};

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
    AddMul(Value, Value, String),
    Out(Value),
    Noop,
}

impl Instruction {
    fn ref_registers(&self) -> HashSet<String> {
        let (values, regs) = match self {
            Instruction::Copy(v, r) => (vec![v], vec![r]),
            Instruction::Inc(r) => (vec![], vec![r]),
            Instruction::Dec(r) => (vec![], vec![r]),
            Instruction::JumpNotZero(v1, v2) => (vec![v1, v2], vec![]),
            Instruction::Toggle(v) => (vec![v], vec![]),
            Instruction::AddMul(v1, v2, r) => (vec![v1, v2], vec![r]),
            Instruction::Out(v) => (vec![v], vec![]),
            Instruction::Noop => (vec![], vec![]),
        };

        values
            .into_iter()
            .flat_map(|v| match v {
                Value::Immediate(_) => None,
                Value::Register(r) => Some(r.clone()),
            })
            .chain(regs.into_iter().cloned())
            .collect()
    }
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub registers: HashMap<String, i32>,
    pub output: Vec<i32>,
    pub pc: usize,
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
                        "out" => Instruction::Out(Value::parse(args)),
                        _ => panic!("Unknown instruction"),
                    }
                })
                .collect_vec(),
            registers: HashMap::new(),
            output: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        self.reset();
        while !self.is_completed() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        // println!("{:?}", &self.instructions[self.pc]);
        match &self.instructions[self.pc] {
            Instruction::Copy(src, trg) => {
                *self.registers.get_mut(trg).unwrap() = src.get(&self.registers);
            }
            Instruction::Inc(reg) => *self.registers.get_mut(reg).unwrap() += 1,
            Instruction::Dec(reg) => *self.registers.get_mut(reg).unwrap() -= 1,
            Instruction::JumpNotZero(cond, offset) => {
                if cond.get(&self.registers) != 0 {
                    self.pc = (self.pc as i32 + offset.get(&self.registers) - 1) as usize
                }
            }

            Instruction::Toggle(off) => {
                let offset = off.get(&self.registers);
                if let Some(inst) = self.instructions.get_mut((self.pc as i32 + offset) as usize) {
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
                        Instruction::Out(_) => panic!("Out and Tgl can't be used together"),
                        Instruction::AddMul(_, _, _) => panic!("Wrong optimization"),
                        Instruction::Noop => panic!("Wrong optimization"),
                    }
                }
            }
            Instruction::Out(v) => self.output.push(v.get(&self.registers)),
            Instruction::AddMul(v1, v2, trg) => {
                *self.registers.get_mut(trg).unwrap() += v1.get(&self.registers) * v2.get(&self.registers);
            }
            Instruction::Noop => (),
        }
        self.pc += 1;
    }

    pub fn is_completed(&self) -> bool {
        self.pc >= self.instructions.len()
    }

    pub fn reset(&mut self) {
        self.registers = self
            .instructions
            .iter()
            .flat_map(|inst| inst.ref_registers())
            .map(|r| (r, 0))
            .collect();
        self.pc = 0;
        self.output.clear();
    }

     // Finds nested for loops which implement multiplication by repeated addition
    pub fn optimize_multiplication(&mut self) -> bool{
        // -> a := a + c*d, c, d := 0
        // cpy b c 
        // inc a   
        // dec c   
        // jnz c -2
        // dec d   
        // jnz d -5
        let mut replacements = Vec::new();
        for (i, slice) in self.instructions[..].windows(6).enumerate() {
            let [
                Instruction::Copy(s1, t1),
                Instruction::Inc(s2), 
                Instruction::Dec(s3),
                Instruction::JumpNotZero(Value::Register(s4), Value::Immediate(-2)), 
                Instruction::Dec(s5),
                Instruction::JumpNotZero(Value::Register(s6), Value::Immediate(-5)),
                ] = slice else { continue; };

            if [t1, s3, s4].iter().all_equal() && [s5, s6].iter().all_equal() {
                replacements.push((
                    i,
                    [
                        vec![Instruction::Noop; 2],
                        vec![
                            Instruction::Copy(s1.clone(), t1.clone()),
                            Instruction::AddMul(Value::Register(s3.clone()), Value::Register(s5.clone()), s2.clone()),
                            Instruction::Copy(Value::Immediate(0), s3.clone()),
                            Instruction::Copy(Value::Immediate(0), s5.clone()),
                        ],
                    ]
                    .concat(),
                ));
            }
        }

        let optimized = !replacements.is_empty();
        for (i, repl) in replacements {
            self.instructions.splice(i..i + repl.len(), repl);
        }

        optimized
    }
}
