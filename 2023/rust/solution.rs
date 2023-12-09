use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub type ProblemInput = Vec<String>;
pub type ProblemResult = i128;

pub trait Solution {
    fn solve(&self, version: u8, use_sample: bool) -> Option<i128> {
        let data = self.get_input(version, use_sample)?;
        match version {
            1 => Some(self.solve_version01(data)),
            2 => Some(self.solve_version02(data)),
            _ => panic!(),
        }
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

    fn solve_version01(&self, input: ProblemInput) -> ProblemResult;
    fn solve_version02(&self, input: ProblemInput) -> ProblemResult;
}
