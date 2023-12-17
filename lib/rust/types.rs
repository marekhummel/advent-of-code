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

pub trait IntoSome<T> {
    fn into_some(self) -> Option<T>;
}

impl IntoSome<ProblemResult> for i128 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::I128(self))
    }
}

impl IntoSome<ProblemResult> for i64 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::I64(self))
    }
}

impl IntoSome<ProblemResult> for i32 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::I32(self))
    }
}

impl IntoSome<ProblemResult> for i16 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::I16(self))
    }
}

impl IntoSome<ProblemResult> for i8 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::I8(self))
    }
}

impl IntoSome<ProblemResult> for u128 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::U128(self))
    }
}

impl IntoSome<ProblemResult> for u64 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::U64(self))
    }
}

impl IntoSome<ProblemResult> for u32 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::U32(self))
    }
}

impl IntoSome<ProblemResult> for u16 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::U16(self))
    }
}

impl IntoSome<ProblemResult> for u8 {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::U8(self))
    }
}

impl IntoSome<ProblemResult> for usize {
    fn into_some(self) -> Option<ProblemResult> {
        Some(ProblemResult::USize(self))
    }
}
