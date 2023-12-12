use std::{
    fmt::Display,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{Duration, Instant},
};

pub type ProblemInput = Vec<String>;

pub trait Solution {
    fn solve(&self, version: u8, use_sample: bool) -> Option<(ProblemResult, Duration)> {
        let data = self.get_input(version, use_sample)?;
        let now = Instant::now();
        let result = match version {
            1 => self.solve_version01(data),
            2 => self.solve_version02(data),
            _ => panic!(),
        };
        result.map(|x| (x, now.elapsed()))
    }

    fn get_input(&self, version: u8, use_sample: bool) -> Option<ProblemInput> {
        let base_filename = if use_sample { "sample" } else { "input" };
        let day = self.get_day();
        let mut fullname = format!("2023\\inputs\\{base_filename}{day:02}.txt");
        if !Path::new(&fullname).exists() {
            fullname = fullname.replace(".txt", format!("_{version}.txt").as_str());
        }

        let file = File::open(fullname).ok()?;
        let buf = BufReader::new(file);
        Some(buf.lines().map(|l| l.expect("Could not parse line")).collect())
    }

    fn get_day(&self) -> u8;

    fn solve_version01(&self, input: ProblemInput) -> Option<ProblemResult>;
    fn solve_version02(&self, input: ProblemInput) -> Option<ProblemResult>;
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
