use itertools::Itertools;

pub struct KnotHash {
    pub circle: Vec<u8>,
    pub lengths: Vec<usize>,
    n: usize,
    i: usize,
    skip: usize,
}

impl KnotHash {
    pub fn new(key: &str) -> Self {
        let key_lengths = key.bytes().map(|b| b as usize);
        KnotHash {
            n: 256,
            lengths: key_lengths.chain([17, 31, 73, 47, 23]).collect_vec(),
            circle: (0..=255).collect_vec(),
            i: 0,
            skip: 0,
        }
    }

    pub fn custom(n: usize, lengths: Vec<usize>) -> Self {
        KnotHash {
            n,
            lengths,
            circle: (0..n).map(|b| b as u8).collect_vec(),
            i: 0,
            skip: 0,
        }
    }

    pub fn hash(&mut self) -> Vec<u8> {
        for _ in 0..64 {
            self.round();
        }

        self.to_dense()
    }

    pub fn round(&mut self) {
        for length in self.lengths.iter() {
            let mut doubled = [self.circle.clone(), self.circle.clone()].concat();
            let reversed = doubled.iter().skip(self.i).take(*length).copied().rev().collect_vec();
            doubled.splice(self.i..self.i + *length, reversed);
            self.circle = [&doubled[self.n..self.n + self.i], &doubled[self.i..self.n]].concat();

            self.i = (self.i + length + self.skip) % self.n;
            self.skip += 1;
        }
    }

    pub fn to_dense(&self) -> Vec<u8> {
        self.circle
            .iter()
            .chunks(16)
            .into_iter()
            .map(|block| block.fold(0, |acc, elem| acc ^ elem))
            .collect_vec()
    }
}
