use core::panic;
use std::collections::{HashMap, VecDeque};

use crate::iterator::ParsedExt;

pub struct Program {
    pub memory: HashMap<usize, i128>,
    pub input: VecDeque<i128>,
    pub output: Vec<i128>,
    pc: usize,
    halted: bool,
    relative_base: i128,
}

impl Program {
    pub fn init(intcode: &str) -> Self {
        Program {
            memory: intcode.split(',').parsed().enumerate().collect(),
            input: VecDeque::new(),
            output: vec![],
            pc: 0,
            halted: false,
            relative_base: 0,
        }
    }

    pub fn execute(&mut self) {
        while !self.halted {
            self.execute_instruction();
        }
    }

    pub fn execute_until_output(&mut self) -> Option<i128> {
        while !self.halted {
            let before = self.output.len();
            self.execute_instruction();

            if self.output.len() > before {
                return self.output.last().copied();
            }
        }

        None
    }

    fn parse_current_opcode(&self) -> (Instruction, Vec<ParameterMode>) {
        let mut opcode = self.read_memory(self.pc);
        let inst: Instruction = (opcode % 100).into();

        opcode /= 100;

        let mut pms = Vec::new();
        for _ in 0..inst.num_parameters() {
            pms.push((opcode % 10).into());
            opcode /= 10;
        }

        (inst, pms)
    }

    fn read_memory(&self, idx: usize) -> i128 {
        *self.memory.get(&idx).unwrap_or(&0)
    }

    fn get_value(&self, narg: usize, pms: &[ParameterMode]) -> i128 {
        let param = self.read_memory(self.pc + narg);
        match pms[narg - 1] {
            ParameterMode::Position => self.read_memory(param as usize),
            ParameterMode::Immediate => param,
            ParameterMode::Relative => self.read_memory((self.relative_base + param) as usize),
        }
    }

    fn get_target(&mut self, narg: usize, pms: &[ParameterMode]) -> &mut i128 {
        let param = self.read_memory(self.pc + narg);
        let index = match pms[narg - 1] {
            ParameterMode::Position => param,
            ParameterMode::Immediate => panic!("invalid target parameter mode"),
            ParameterMode::Relative => self.relative_base + param,
        };

        self.memory.entry(index as usize).or_insert(0)
    }

    fn execute_instruction(&mut self) {
        let (inst, pms) = self.parse_current_opcode();
        let mut pc_modified = false;

        match inst {
            Instruction::Halt => self.halted = true,
            Instruction::Add | Instruction::Mul | Instruction::LessThan | Instruction::Equals => {
                let arg1 = self.get_value(1, &pms);
                let arg2 = self.get_value(2, &pms);
                let target = self.get_target(3, &pms);
                match inst {
                    Instruction::Add => *target = arg1 + arg2,
                    Instruction::Mul => *target = arg1 * arg2,
                    Instruction::LessThan => *target = (arg1 < arg2) as i128,
                    Instruction::Equals => *target = (arg1 == arg2) as i128,
                    _ => unreachable!(),
                }
            }
            Instruction::Input => {
                let input_value = self.input.pop_front().expect("No inputs left");
                let target = self.get_target(1, &pms);
                *target = input_value;
            }
            Instruction::Output => {
                let value = self.get_value(1, &pms);
                self.output.push(value);
            }
            Instruction::JmpTrue | Instruction::JmpFalse => {
                let condition = self.get_value(1, &pms);
                let jump = self.get_value(2, &pms) as usize;

                pc_modified = true;
                match inst {
                    Instruction::JmpTrue if condition != 0 => self.pc = jump,
                    Instruction::JmpFalse if condition == 0 => self.pc = jump,
                    _ => pc_modified = false,
                }
            }
            Instruction::AdjRelBase => {
                let offset = self.get_value(1, &pms);
                self.relative_base += offset;
            }
        }

        if !pc_modified {
            let params = inst.num_parameters();
            self.pc += params + 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Add,
    Mul,
    Input,
    Output,
    JmpTrue,
    JmpFalse,
    LessThan,
    Equals,
    AdjRelBase,
    Halt,
}

impl Instruction {
    fn num_parameters(&self) -> usize {
        match self {
            Instruction::Add => 3,
            Instruction::Mul => 3,
            Instruction::Input => 1,
            Instruction::Output => 1,
            Instruction::JmpTrue => 2,
            Instruction::JmpFalse => 2,
            Instruction::LessThan => 3,
            Instruction::Equals => 3,
            Instruction::AdjRelBase => 1,
            Instruction::Halt => 0,
        }
    }
}

impl From<i128> for Instruction {
    fn from(value: i128) -> Self {
        match value {
            1 => Instruction::Add,
            2 => Instruction::Mul,
            3 => Instruction::Input,
            4 => Instruction::Output,
            5 => Instruction::JmpTrue,
            6 => Instruction::JmpFalse,
            7 => Instruction::LessThan,
            8 => Instruction::Equals,
            9 => Instruction::AdjRelBase,
            99 => Instruction::Halt,
            _ => panic!("Invalid opcode: '{value}'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i128> for ParameterMode {
    fn from(value: i128) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Invalid parameter mode"),
        }
    }
}
