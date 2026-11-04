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
        Grid::new(self.lines.iter().map(|row| row.chars().collect()).collect())
    }
}

#[derive(Debug, PartialEq, Eq)]
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
    ISize(isize),
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
            ProblemResult::ISize(v) => write!(f, "{}", v),
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

macro_rules! impl_to_result {
    ($e:path,$t:ty) => {
        impl ToResult for $t {
            fn to_result(self) -> ProblemResult {
                $e(self)
            }
        }
    };
}

impl_to_result!(ProblemResult::I128, i128);
impl_to_result!(ProblemResult::I64, i64);
impl_to_result!(ProblemResult::I32, i32);
impl_to_result!(ProblemResult::I16, i16);
impl_to_result!(ProblemResult::I8, i8);

impl_to_result!(ProblemResult::U128, u128);
impl_to_result!(ProblemResult::U64, u64);
impl_to_result!(ProblemResult::U32, u32);
impl_to_result!(ProblemResult::U16, u16);
impl_to_result!(ProblemResult::U8, u8);

impl_to_result!(ProblemResult::USize, usize);
impl_to_result!(ProblemResult::ISize, isize);
impl_to_result!(ProblemResult::BigInt, BigInt);
impl_to_result!(ProblemResult::String, String);

impl ToResult for &str {
    fn to_result(self) -> ProblemResult {
        ProblemResult::String(self.to_string())
    }
}
