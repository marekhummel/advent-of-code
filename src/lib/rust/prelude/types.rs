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
    NoSample,
    Unsolved,
    NotImplemented,
    NoPartTwo,
    Signed(i128),
    Unsigned(u128),
    BigInt(BigInt),
    String(String),
}

impl Display for ProblemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemResult::Signed(v) => write!(f, "{}", v),
            ProblemResult::Unsigned(v) => write!(f, "{}", v),
            ProblemResult::BigInt(v) => write!(f, "{}", v),
            ProblemResult::String(v) => write!(f, "{}", v),
            ProblemResult::NoSample => write!(f, "<No Sample Defined>"),
            ProblemResult::Unsolved => write!(f, "<Unsolved>"),
            ProblemResult::NotImplemented => write!(f, "<Not Implemented>"),
            ProblemResult::NoPartTwo => write!(f, "<No Part Two>"),
        }
    }
}

pub trait ToResult {
    fn to_result(self) -> ProblemResult;
}

macro_rules! impl_integer_result {
    ($variant:ident, $storage:ty: $($source:ty),+ $(,)?) => {
        $(
            impl ToResult for $source {
                fn to_result(self) -> ProblemResult {
                    ProblemResult::$variant(self as $storage)
                }
            }
        )+
    };
}

impl_integer_result!(Signed, i128: i8, i16, i32, i64, isize);
impl_integer_result!(Unsigned, u128: u8, u16, u32, u64, usize);

impl ToResult for i128 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::Signed(self)
    }
}

impl ToResult for u128 {
    fn to_result(self) -> ProblemResult {
        ProblemResult::Unsigned(self)
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
        ProblemResult::String(self.to_owned())
    }
}
