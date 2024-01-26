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

impl Instruction {
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
