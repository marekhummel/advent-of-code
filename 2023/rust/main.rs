mod solution;
mod solutions;

use solutions::day01;
use solutions::day02;
use solutions::day03;
use solutions::day04;
use solutions::day05;
use solutions::day06;
use solutions::day07;
use solutions::day08;
use solutions::day09;
use solutions::day10;
use solutions::day11;
use solutions::day12;
use solutions::day13;
use solutions::day14;
use solutions::day15;
use solutions::day16;
use solutions::day17;
use solutions::day18;
use solutions::day19;
use solutions::day20;
use solutions::day21;
use solutions::day22;
use solutions::day23;
use solutions::day24;
use std::collections::HashMap;
use std::env;
use std::time::Duration;

use crate::solution::Solution;

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(day01::Solution01 {}),
        Box::new(day02::Solution02 {}),
        Box::new(day03::Solution03 {}),
        Box::new(day04::Solution04 {}),
        Box::new(day05::Solution05 {}),
        Box::new(day06::Solution06 {}),
        Box::new(day07::Solution07 {}),
        Box::new(day08::Solution08 {}),
        Box::new(day09::Solution09 {}),
        Box::new(day10::Solution10 {}),
        Box::new(day11::Solution11 {}),
        Box::new(day12::Solution12 {}),
        Box::new(day13::Solution13 {}),
        Box::new(day14::Solution14 {}),
        Box::new(day15::Solution15 {}),
        Box::new(day16::Solution16 {}),
        Box::new(day17::Solution17 {}),
        Box::new(day18::Solution18 {}),
        Box::new(day19::Solution19 {}),
        Box::new(day20::Solution20 {}),
        Box::new(day21::Solution21 {}),
        Box::new(day22::Solution22 {}),
        Box::new(day23::Solution23 {}),
        Box::new(day24::Solution24 {}),
    ];

    let arg = env::args().nth(1).expect("Pass day or 'main' as argument!");
    let sample_str = HashMap::from([(false, "real"), (true, "samp")]);

    match arg.as_str() {
        "main" => {
            let mut total_time = Duration::ZERO;
            for s in &solutions {
                println!("Day {0:02}:", s.get_day());
                for version in [1, 2] {
                    for sample in [true, false] {
                        match s.solve(version, sample) {
                            Some((v, e)) => {
                                println!("  V{version} {0}:  {v}", sample_str[&sample]);
                                total_time += e;
                            }
                            None => println!("  V{version} {0}:  <Unsolved>", sample_str[&sample]),
                        }
                    }
                }
            }

            println!("\n\nTotal Runtime: {total_time:?}");
        }
        _ => {
            let day = arg
                .strip_prefix("day")
                .expect("Argument should start with 'day'")
                .parse::<usize>()
                .expect("Argument should have format 'dayXX' with XX being a valid number!");

            let s = &solutions[day - 1];

            match ALL {
                true => {
                    let mut total_time = Duration::ZERO;
                    for version in [1, 2] {
                        for sample in [true, false] {
                            let (v, e) = s.solve(version, sample).unwrap();
                            total_time += e;
                            println!("V{version} {0}:  {v}", sample_str[&sample]);
                        }
                    }
                    println!("\nTotal Runtime: {total_time:?}");
                }
                false => {
                    let (v, e) = s
                        .solve(VERSION, USE_SAMPLE)
                        .map_or((String::from("<Unsolved>"), Duration::ZERO), |(v, e)| {
                            (v.to_string(), e)
                        });
                    println!(
                        "Day {day:02} / Version {VERSION} / Data '{0}' => {e:?}\n{v}",
                        sample_str[&USE_SAMPLE]
                    );
                }
            }
        }
    }
}
