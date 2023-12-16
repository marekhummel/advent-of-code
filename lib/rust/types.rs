use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

// pub type ProblemInput = Vec<String>;

pub struct ProblemInput {
    lines: Vec<String>,
}

impl ProblemInput {
    pub fn read(filename: &str) -> Option<Self> {
        let file = File::open(filename).ok()?;
        let buf = BufReader::new(file);

        let lines = buf.lines().map(|l| l.expect("Could not parse line")).collect();
        Some(ProblemInput { lines })
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines.iter().cloned().collect_vec()
    }

    pub fn string(&self) -> String {
        self.lines.join("")
    }

    pub fn grid(&self) -> Vec<Vec<char>> {
        self.lines.iter().map(|row| row.chars().collect_vec()).collect_vec()
    }

    pub fn enumerated_grid(&self) -> Vec<Vec<(usize, usize, char)>> {
        self.lines
            .iter()
            .enumerate()
            .map(|(y, row)| row.chars().enumerate().map(|(x, c)| (y, x, c)).collect_vec())
            .collect_vec()
    }

    pub fn grid_size(&self) -> (usize, usize) {
        (self.lines.len(), self.lines[0].len())
    }
}

pub enum ProblemResult {
    I128(i128),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    U128(u128),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    USize(usize),
}

impl Display for ProblemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemResult::I128(v) => write!(f, "{}", v),
            ProblemResult::I64(v) => write!(f, "{}", v),
            ProblemResult::I32(v) => write!(f, "{}", v),
            ProblemResult::I16(v) => write!(f, "{}", v),
            ProblemResult::I8(v) => write!(f, "{}", v),
            ProblemResult::U128(v) => write!(f, "{}", v),
            ProblemResult::U64(v) => write!(f, "{}", v),
            ProblemResult::U32(v) => write!(f, "{}", v),
            ProblemResult::U16(v) => write!(f, "{}", v),
            ProblemResult::U8(v) => write!(f, "{}", v),
            ProblemResult::USize(v) => write!(f, "{}", v),
        }
    }
}

impl From<i128> for ProblemResult {
    fn from(val: i128) -> Self {
        ProblemResult::I128(val)
    }
}

impl From<i64> for ProblemResult {
    fn from(val: i64) -> Self {
        ProblemResult::I64(val)
    }
}

impl From<i32> for ProblemResult {
    fn from(val: i32) -> Self {
        ProblemResult::I32(val)
    }
}

impl From<i16> for ProblemResult {
    fn from(val: i16) -> Self {
        ProblemResult::I16(val)
    }
}

impl From<i8> for ProblemResult {
    fn from(val: i8) -> Self {
        ProblemResult::I8(val)
    }
}

impl From<u128> for ProblemResult {
    fn from(val: u128) -> Self {
        ProblemResult::U128(val)
    }
}

impl From<u64> for ProblemResult {
    fn from(val: u64) -> Self {
        ProblemResult::U64(val)
    }
}

impl From<u32> for ProblemResult {
    fn from(val: u32) -> Self {
        ProblemResult::U32(val)
    }
}

impl From<u16> for ProblemResult {
    fn from(val: u16) -> Self {
        ProblemResult::U16(val)
    }
}

impl From<u8> for ProblemResult {
    fn from(val: u8) -> Self {
        ProblemResult::U8(val)
    }
}

impl From<usize> for ProblemResult {
    fn from(val: usize) -> Self {
        ProblemResult::USize(val)
    }
}
