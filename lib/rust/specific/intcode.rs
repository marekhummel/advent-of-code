use core::panic;
use std::collections::VecDeque;

pub struct Program {
    pub intcode: Vec<i32>,
    pub input: VecDeque<i32>,
    pub output: Vec<i32>,
    pc: usize,
    halted: bool,
}

impl Program {
    pub fn init(intcode: Vec<i32>) -> Self {
        Program {
            intcode,
            input: VecDeque::new(),
            output: vec![],
            pc: 0,
            halted: false,
        }
    }

    pub fn execute(&mut self) {
        while !self.halted {
            self.execute_instruction();
        }
    }

    pub fn execute_until_output(&mut self) -> Option<i32> {
        while !self.halted {
            let before = self.output.len();
            self.execute_instruction();

            if self.output.len() > before {
                return Some(*self.output.last().unwrap());
            }
        }

        None
    }

    fn parse_opcode(&self, mut opcode: i32) -> (Instruction, Vec<ParameterMode>) {
        let inst: Instruction = (opcode % 100).into();

        opcode /= 100;

        let mut pms = Vec::new();
        for _ in 0..inst.num_parameters() {
            pms.push((opcode % 10).into());
            opcode /= 10;
        }

        (inst, pms)
    }

    fn get_value(&self, narg: usize, pc: usize, pms: &[ParameterMode]) -> i32 {
        let param = self.intcode[pc + narg];
        match pms[narg - 1] {
            ParameterMode::Position => self.intcode[param as usize],
            ParameterMode::Immediate => param,
        }
    }

    fn execute_instruction(&mut self) {
        let (inst, pms) = self.parse_opcode(self.intcode[self.pc]);
        let mut pc_modified = false;

        match inst {
            Instruction::Halt => self.halted = true,
            Instruction::Add | Instruction::Mul | Instruction::LessThan | Instruction::Equals => {
                let arg1 = self.get_value(1, self.pc, &pms);
                let arg2 = self.get_value(2, self.pc, &pms);
                let target_pc = self.intcode[self.pc + 3] as usize;
                let target = self.intcode.get_mut(target_pc).unwrap();
                match inst {
                    Instruction::Add => *target = arg1 + arg2,
                    Instruction::Mul => *target = arg1 * arg2,
                    Instruction::LessThan => *target = (arg1 < arg2) as i32,
                    Instruction::Equals => *target = (arg1 == arg2) as i32,
                    _ => unreachable!(),
                }
            }
            Instruction::Input => {
                let target = self.intcode[self.pc + 1] as usize;
                *self.intcode.get_mut(target).unwrap() = self.input.pop_front().expect("No inputs left");
            }
            Instruction::Output => {
                let value = self.get_value(1, self.pc, &pms);
                self.output.push(value);
            }
            Instruction::JmpTrue | Instruction::JmpFalse => {
                let condition = self.get_value(1, self.pc, &pms);
                let jump = self.get_value(2, self.pc, &pms) as usize;

                pc_modified = true;
                match inst {
                    Instruction::JmpTrue if condition != 0 => self.pc = jump,
                    Instruction::JmpFalse if condition == 0 => self.pc = jump,
                    _ => pc_modified = false,
                }
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
            Instruction::Halt => 0,
        }
    }
}

impl From<i32> for Instruction {
    fn from(value: i32) -> Self {
        match value {
            1 => Instruction::Add,
            2 => Instruction::Mul,
            3 => Instruction::Input,
            4 => Instruction::Output,
            5 => Instruction::JmpTrue,
            6 => Instruction::JmpFalse,
            7 => Instruction::LessThan,
            8 => Instruction::Equals,
            99 => Instruction::Halt,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(value: i32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!(),
        }
    }
}
