use std::path::Path;

use clap::Parser;

use crate::prelude::{
    solution::Solution,
    types::{ProblemInput, ProblemResult},
};

pub struct AocRunner {
    pub year: u16,
    pub solutions: Vec<Box<dyn Solution>>,
}

#[derive(Parser)]
#[command(about = "Print one Advent of Code answer")]
struct Cli {
    /// Day number (1-25)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    /// Part number (1 or 2)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,
    /// Use sample input instead of real input.
    #[arg(long)]
    sample: bool,
}

impl AocRunner {
    pub fn parse_and_run(&self) {
        let args = Cli::parse();
        match self.run(args.day, args.part, args.sample) {
            Ok(result) => println!("{result}"),
            Err(error) => {
                eprintln!("Error: {error}");
                std::process::exit(1);
            }
        }
    }

    fn run(&self, day: u8, part: u8, sample: bool) -> Result<ProblemResult, String> {
        let solution = self
            .solutions
            .get(usize::from(day - 1))
            .ok_or_else(|| format!("No solution for {} day {day:02}", self.year))?;
        let prefix = if sample { "sample" } else { "input" };
        let mut filename = format!("inputs/{}/{prefix}{day:02}.txt", self.year);
        if !Path::new(&filename).exists() {
            filename = format!("inputs/{}/{prefix}{day:02}_{part}.txt", self.year);
        }
        let input = ProblemInput::read(&filename).ok_or_else(|| format!("Could not read {filename}"))?;
        let result = match part {
            1 => solution.solve_version01(input, sample),
            2 => solution.solve_version02(input, sample),
            _ => unreachable!(),
        };
        match result {
            ProblemResult::NoInput | ProblemResult::NoSample | ProblemResult::Unsolved | ProblemResult::NoPartTwo => {
                Err(result.to_string())
            }
            _ => Ok(result),
        }
    }
}
