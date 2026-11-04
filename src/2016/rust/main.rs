mod solutions;

use std::collections::HashMap;

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
use solutions::day25;
// aoc:imports

fn create_runner() -> AocRunner {
    let entries: [(u8, Box<dyn Solution>); _] = [
        (1, Box::new(day01::Solution01 {})),
        (2, Box::new(day02::Solution02 {})),
        (3, Box::new(day03::Solution03 {})),
        (4, Box::new(day04::Solution04 {})),
        (5, Box::new(day05::Solution05 {})),
        (6, Box::new(day06::Solution06 {})),
        (7, Box::new(day07::Solution07 {})),
        (8, Box::new(day08::Solution08 {})),
        (9, Box::new(day09::Solution09 {})),
        (10, Box::new(day10::Solution10 {})),
        (11, Box::new(day11::Solution11 {})),
        (12, Box::new(day12::Solution12 {})),
        (13, Box::new(day13::Solution13 {})),
        (14, Box::new(day14::Solution14 {})),
        (15, Box::new(day15::Solution15 {})),
        (16, Box::new(day16::Solution16 {})),
        (17, Box::new(day17::Solution17 {})),
        (18, Box::new(day18::Solution18 {})),
        (19, Box::new(day19::Solution19 {})),
        (20, Box::new(day20::Solution20 {})),
        (21, Box::new(day21::Solution21 {})),
        (22, Box::new(day22::Solution22 {})),
        (23, Box::new(day23::Solution23 {})),
        (24, Box::new(day24::Solution24 {})),
        (25, Box::new(day25::Solution25 {})),
        // aoc:solutions
    ];

    AocRunner {
        year: 2016,
        solutions: HashMap::from(entries),
    }
}

fn main() {
    let runner = create_runner();
    runner.parse_and_run();
}
