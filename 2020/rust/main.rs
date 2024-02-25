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
        //
    ];

    AocRunner { year: 2020, solutions }
}

fn main() {
    let arg = env::args().nth(1);
    let runner = create_runner();
    runner.run(arg, ALL, VERSION, USE_SAMPLE);
}

#[cfg(test)]
mod tests2020 {
    use aoc_lib::test_day;

    test_day!(day01);
    test_day!(day02);
    test_day!(day03);
    test_day!(day04);
    test_day!(day05);
    test_day!(day06);
    test_day!(day07);
    test_day!(day08);
    test_day!(day09);
    test_day!(day10);
    test_day!(day11);
    test_day!(day12);
    test_day!(day13);
    test_day!(day14);
    test_day!(day15);
    test_day!(day16);
    test_day!(day17);
    test_day!(day18);
    test_day!(day19);
    test_day!(day20);
    test_day!(day21);
    test_day!(day22);
    test_day!(day23);
}
