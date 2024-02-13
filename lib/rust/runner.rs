use std::{path::Path, time::Duration};

use crate::{
    solution::Solution,
    types::{ProblemInput, ProblemResult},
};

pub struct AocRunner {
    pub year: u16,
    pub solutions: Vec<Box<dyn Solution>>,
}

impl AocRunner {
    const SAMPLE_STR: [&'static str; 2] = ["real", "samp"];

    pub fn run(&self, env_arg: Option<String>, full_day: bool, version: u8, use_sample: bool) {
        let arg = env_arg.expect("Pass day or 'main' as argument!");

        match arg.as_str() {
            "main" => self.run_full_year(),
            _ => {
                let day = arg
                    .strip_prefix("day")
                    .expect("Argument should start with 'day'")
                    .parse::<usize>()
                    .expect("Argument should have format 'dayXX' with XX being a valid number!");

                match full_day {
                    true => self.run_day(day),
                    false => self.run_single(day, version, use_sample),
                }
            }
        }
    }

    fn run_full_year(&self) {
        let mut total_time = Duration::ZERO;
        for day in 0..self.solutions.len() {
            println!("Day {0:02}:", day + 1);
            for version in [1, 2] {
                for use_sample in [true, false] {
                    let (result, elapsed) = self.get_result(day + 1, version, use_sample);
                    total_time += elapsed;
                    println!("  V{version} {}:  {result}", Self::SAMPLE_STR[use_sample as usize]);
                }
            }
            println!()
        }

        println!("\n\nTotal Runtime: {total_time:?}");
    }

    fn run_day(&self, day: usize) {
        let mut total_time = Duration::ZERO;
        for version in [1, 2] {
            for use_sample in [true, false] {
                let (result, elapsed) = self.get_result(day, version, use_sample);
                total_time += elapsed;
                println!("V{version} {}:  {result}", Self::SAMPLE_STR[use_sample as usize]);
            }
        }
        println!("\nTotal Runtime: {total_time:?}");
    }

    fn run_single(&self, day: usize, version: u8, use_sample: bool) {
        if self.solutions.len() < day {
            println!("No solution implemented for day {day:02} in year {0}", self.year);
            return;
        }

        let (result, elapsed) = self.get_result(day, version, use_sample);
        println!(
            "Day {day:02} / Version {version} / Data '{}' => {:?}\n{}",
            Self::SAMPLE_STR[use_sample as usize],
            elapsed,
            result
        );
    }

    fn get_input(&self, year: u16, day: u8, version: u8, use_sample: bool) -> Option<ProblemInput> {
        let base_filename = if use_sample { "sample" } else { "input" };
        let mut fullname = format!("{year}\\inputs\\{base_filename}{day:02}.txt");
        if !Path::new(&fullname).exists() {
            fullname = fullname.replace(".txt", format!("_{version}.txt").as_str());
        }

        ProblemInput::read(&fullname)
    }

    fn get_result(&self, day: usize, version: u8, use_sample: bool) -> (ProblemResult, Duration) {
        let s = &self.solutions[day - 1];
        match self.get_input(self.year, day as u8, version, use_sample) {
            Some(input) => s.solve(input, version, use_sample),
            None => (ProblemResult::NoInput, Duration::ZERO),
        }
    }
}
