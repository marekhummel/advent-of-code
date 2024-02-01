pub struct Program {
    pub intcode: Vec<u32>,
}

impl Program {
    pub fn execute(&mut self) {
        let mut pos = 0;
        loop {
            match self.intcode[pos] {
                99 => break,
                opcode => {
                    let target = self.intcode[pos + 3] as usize;
                    let arg1 = self.intcode[self.intcode[pos + 1] as usize];
                    let arg2 = self.intcode[self.intcode[pos + 2] as usize];
                    match opcode {
                        1 => *self.intcode.get_mut(target).unwrap() = arg1 + arg2,
                        2 => *self.intcode.get_mut(target).unwrap() = arg1 * arg2,
                        _ => unreachable!(),
                    }
                }
            }

            pos += 4;
        }
    }
}
