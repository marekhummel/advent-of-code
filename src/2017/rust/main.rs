mod solutions;

use std::collections::HashMap;

use aoc_lib::prelude::runner::AocRunner;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::solution;

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
    let entries: [(u8, Solution); _] = [
        (1, solution!(day01)),
        (2, solution!(day02)),
        (3, solution!(day03)),
        (4, solution!(day04)),
        (5, solution!(day05)),
        (6, solution!(day06)),
        (7, solution!(day07)),
        (8, solution!(day08)),
        (9, solution!(day09)),
        (10, solution!(day10)),
        (11, solution!(day11)),
        (12, solution!(day12)),
        (13, solution!(day13)),
        (14, solution!(day14)),
        (15, solution!(day15)),
        (16, solution!(day16)),
        (17, solution!(day17)),
        (18, solution!(day18)),
        (19, solution!(day19)),
        (20, solution!(day20)),
        (21, solution!(day21)),
        (22, solution!(day22)),
        (23, solution!(day23)),
        (24, solution!(day24)),
        (25, solution!(day25)),
        // aoc:solutions
    ];

    AocRunner {
        year: 2017,
        solutions: HashMap::from(entries),
    }
}

fn main() {
    let runner = create_runner();
    runner.parse_and_run();
}
