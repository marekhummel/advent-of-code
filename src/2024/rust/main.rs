mod solutions;

use aoc_lib::prelude::runner::AocRunner;
use aoc_lib::prelude::solution::Solution;

use solutions::day01;
use solutions::day02;
use solutions::day03;
use solutions::day04;
use solutions::day05;
use solutions::day06;
// aoc:imports

fn create_runner() -> AocRunner {
    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(day01::Solution01 {}),
        Box::new(day02::Solution02 {}),
        Box::new(day03::Solution03 {}),
        Box::new(day04::Solution04 {}),
        Box::new(day05::Solution05 {}),
        Box::new(day06::Solution06 {}),
        // aoc:solutions
    ];

    AocRunner { year: 2024, solutions }
}

fn main() {
    let runner = create_runner();
    runner.parse_and_run();
}
