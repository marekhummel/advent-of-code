mod solution;
mod solutions;

use solutions::day01;
use std::env;

use crate::solution::Solution;

const VERSION: u8 = 2;
const USE_SAMPLE: bool = false;

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Pass day as argument!")
        .strip_prefix("day")
        .expect("Argument should start with 'day'")
        .parse::<usize>()
        .expect("Argument should have format 'dayXX' with XX being a valid number!");

    let solutions = vec![day01::Solution01 {}];
    let s = &solutions[day - 1];
    let v = s.solve(VERSION, USE_SAMPLE);
    println!("Day {day}:  {v}");
}
