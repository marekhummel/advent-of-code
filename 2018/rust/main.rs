mod solutions;

use aoc_lib::runner::AocRunner;
use solutions::day01;
use solutions::day02;
use solutions::day03;
use solutions::day04;
use std::env;

use aoc_lib::solution::Solution;

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(day01::Solution01 {}),
        Box::new(day02::Solution02 {}),
        Box::new(day03::Solution03 {}),
        Box::new(day04::Solution04 {}),
        //
    ];

    let arg = env::args().nth(1);
    let runner = AocRunner { year: 2018, solutions };
    runner.run(arg, ALL, VERSION, USE_SAMPLE);
}
