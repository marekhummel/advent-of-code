use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use num::bigint::BigInt;

use crate::cartesian::Grid;

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

    pub fn grid(&self) -> Grid<char> {
        Grid::new(self.lines.iter().map(|row| row.chars().collect_vec()).collect_vec())
    }

    // pub fn enumerated_grid(&self) -> EnumeratedGrid {
    //     self.lines
    //         .iter()
    //         .enumerate()
    //         .map(|(j, row)| row.chars().enumerate().map(|(i, c)| (Index { i, j }, c)).collect_vec())
    //         .collect_vec()
    // }
}

pub enum ProblemResult {
    NoInput,
    NoSample,
    Unsolved,
    NoPartTwo,
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
    BigInt(BigInt),
    String(String),
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
            ProblemResult::BigInt(v) => write!(f, "{}", v),
            ProblemResult::String(v) => write!(f, "{}", v),
            ProblemResult::NoInput => write!(f, "<No Input Available>"),
            ProblemResult::NoSample => write!(f, "<No Sample Defined>"),
            ProblemResult::Unsolved => write!(f, "<No Solution Implemented>"),
            ProblemResult::NoPartTwo => write!(f, "<No Part Two>"),
        }
    }
}

pub trait ToResult {
    fn to_result(self) -> ProblemResult;
}

impl ToResult for i128 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::I128(self)
    }
}

impl ToResult for i64 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::I64(self)
    }
}

impl ToResult for i32 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::I32(self)
    }
}

impl ToResult for i16 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::I16(self)
    }
}

impl ToResult for i8 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::I8(self)
    }
}

impl ToResult for u128 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::U128(self)
    }
}

impl ToResult for u64 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::U64(self)
    }
}

impl ToResult for u32 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::U32(self)
    }
}

impl ToResult for u16 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::U16(self)
    }
}

impl ToResult for u8 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::U8(self)
    }
}

impl ToResult for usize {
    fn to_result(self) -> ProblemResult {
        ProblemResult::USize(self)
    }
}

impl ToResult for BigInt {
    fn to_result(self) -> ProblemResult {
        ProblemResult::BigInt(self)
    }
}

impl ToResult for String {
    fn to_result(self) -> ProblemResult {
        ProblemResult::String(self)
    }
}

impl ToResult for &str {
    fn to_result(self) -> ProblemResult {
        ProblemResult::String(self.to_string())
    }
}
