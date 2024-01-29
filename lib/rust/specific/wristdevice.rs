use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Addi,
    Addr,
    Muli,
    Mulr,
    Bani,
    Banr,
    Bori,
    Borr,
    Seti,
    Setr,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

pub struct Instruction {
    pub opcode: Op,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rc = (self.c as u8 + b'A') as char;
        let ra = (self.a as u8 + b'A') as char;
        let rb = (self.b as u8 + b'A') as char;

        match self.opcode {
            Op::Addi => write!(f, "{} = {} + {}", rc, ra, self.b),
            Op::Addr => write!(f, "{} = {} + {}", rc, ra, rb),
            Op::Muli => write!(f, "{} = {} * {}", rc, ra, self.b),
            Op::Mulr => write!(f, "{} = {} * {}", rc, ra, rb),
            Op::Bani => write!(f, "{} = {} & {}", rc, ra, self.b),
            Op::Banr => write!(f, "{} = {} & {}", rc, ra, rb),
            Op::Bori => write!(f, "{} = {} | {}", rc, ra, self.b),
            Op::Borr => write!(f, "{} = {} | {}", rc, ra, rb),
            Op::Seti => write!(f, "{} = {}", rc, self.a),
            Op::Setr => write!(f, "{} = {}", rc, ra),
            Op::Gtir => write!(f, "{} = ({} > {})", rc, self.a, rb),
            Op::Gtri => write!(f, "{} = ({} > {})", rc, ra, self.b),
            Op::Gtrr => write!(f, "{} = ({} > {})", rc, ra, rb),
            Op::Eqir => write!(f, "{} = ({} == {})", rc, self.a, rb),
            Op::Eqri => write!(f, "{} = ({} == {})", rc, ra, self.b),
            Op::Eqrr => write!(f, "{} = ({} == {})", rc, ra, rb),
        }
    }
}

impl Instruction {
    pub fn from_line(line: &str) -> Self {
        let (op_str, a, b, c) = line.split_whitespace().collect_tuple().unwrap();
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
    }

    pub fn execute(&self, registers: &mut [usize]) {
        *registers.get_mut(self.c).unwrap() = match self.opcode {
            Op::Addi => registers[self.a] + self.b,
            Op::Addr => registers[self.a] + registers[self.b],
            Op::Muli => registers[self.a] * self.b,
            Op::Mulr => registers[self.a] * registers[self.b],
            Op::Bani => registers[self.a] & self.b,
            Op::Banr => registers[self.a] & registers[self.b],
            Op::Bori => registers[self.a] | self.b,
            Op::Borr => registers[self.a] | registers[self.b],
            Op::Seti => self.a,
            Op::Setr => registers[self.a],
            Op::Gtir => (self.a > registers[self.b]) as usize,
            Op::Gtri => (registers[self.a] > self.b) as usize,
            Op::Gtrr => (registers[self.a] > registers[self.b]) as usize,
            Op::Eqir => (self.a == registers[self.b]) as usize,
            Op::Eqri => (registers[self.a] == self.b) as usize,
            Op::Eqrr => (registers[self.a] == registers[self.b]) as usize,
        }
    }
}

impl Op {
    pub fn iter() -> impl Iterator<Item = Op> {
        [
            Op::Addi,
            Op::Addr,
            Op::Muli,
            Op::Mulr,
            Op::Bani,
            Op::Banr,
            Op::Bori,
            Op::Borr,
            Op::Seti,
            Op::Setr,
            Op::Gtir,
            Op::Gtri,
            Op::Gtrr,
            Op::Eqir,
            Op::Eqri,
            Op::Eqrr,
        ]
        .iter()
        .copied()
    }
}
