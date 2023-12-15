use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use crate::{solution::Solution, types::ProblemInput};

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
        for (day, s) in self.solutions.iter().enumerate() {
            println!("Day {0:02}:", day + 1);
            for version in [1, 2] {
                for sample in [true, false] {
                    let data = self.get_input(self.year, day as u8 + 1, version, sample);
                    match data {
                        Some(input) => match s.solve(input, version) {
                            Some((v, e)) => {
                                println!("  V{version} {0}:  {v}", Self::SAMPLE_STR[sample as usize]);
                                total_time += e;
                            }
                            None => println!("  V{version} {0}:  <Unsolved>", Self::SAMPLE_STR[sample as usize]),
                        },
                        None => println!("  V{version} {0}:  <No Input>", Self::SAMPLE_STR[sample as usize]),
                    }
                }
            }
        }

        println!("\n\nTotal Runtime: {total_time:?}");
    }

    fn run_day(&self, day: usize) {
        let s = &self.solutions[day - 1];
        let mut total_time = Duration::ZERO;
        for version in [1, 2] {
            for sample in [true, false] {
                let input = self.get_input(self.year, day as u8, version, sample).unwrap();
                let (v, e) = s.solve(input, version).unwrap();
                total_time += e;
                println!("V{version} {0}:  {v}", Self::SAMPLE_STR[sample as usize]);
            }
        }
        println!("\nTotal Runtime: {total_time:?}");
    }

    fn run_single(&self, day: usize, version: u8, use_sample: bool) {
        let s = &self.solutions[day - 1];
        let input = self.get_input(self.year, day as u8, version, use_sample).unwrap();
        let (v, e) = s
            .solve(input, version)
            .map_or((String::from("<Unsolved>"), Duration::ZERO), |(v, e)| {
                (v.to_string(), e)
            });
        println!(
            "Day {day:02} / Version {version} / Data '{0}' => {e:?}\n{v}",
            Self::SAMPLE_STR[use_sample as usize]
        );
    }

    fn get_input(&self, year: u16, day: u8, version: u8, use_sample: bool) -> Option<ProblemInput> {
        let base_filename = if use_sample { "sample" } else { "input" };
        let mut fullname = format!("{year}\\inputs\\{base_filename}{day:02}.txt");
        if !Path::new(&fullname).exists() {
            fullname = fullname.replace(".txt", format!("_{version}.txt").as_str());
        }

        let file = File::open(fullname).ok()?;
        let buf = BufReader::new(file);
        Some(buf.lines().map(|l| l.expect("Could not parse line")).collect())
    }
}
