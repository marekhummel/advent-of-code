mod solutions;

use aoc_lib::prelude::runner::AocRunner;
use aoc_lib::prelude::solution::Solution;

use solutions::day01;
use solutions::day02;
use solutions::day03;
use solutions::day04;
use solutions::day05;
use solutions::day06;
use solutions::day07;
use solutions::day08;
use std::env;

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

fn create_runner() -> AocRunner {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(day01::Solution01 {}),
        Box::new(day02::Solution02 {}),
        Box::new(day03::Solution03 {}),
        Box::new(day04::Solution04 {}),
        Box::new(day05::Solution05 {}),
        Box::new(day06::Solution06 {}),
        Box::new(day07::Solution07 {}),
        Box::new(day08::Solution08 {}),
        //
    ];

    AocRunner { year: 2021, solutions }
}

fn main() {
    let arg = env::args().nth(1);
    let runner = create_runner();
    runner.run(arg, ALL, VERSION, USE_SAMPLE);
}

#[cfg(test)]
mod tests2021 {
    use aoc_lib::test_day;

    test_day!(day01);
    test_day!(day02);
    test_day!(day03);
    test_day!(day04);
    test_day!(day05);
    test_day!(day06);
    test_day!(day07);
    test_day!(day08);
    //
}
