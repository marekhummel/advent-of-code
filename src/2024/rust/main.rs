mod solutions;

use std::collections::HashMap;

use aoc_lib::prelude::runner::AocRunner;
use aoc_lib::prelude::solution::Solution;

use solutions::day06;
// aoc:imports

fn create_runner() -> AocRunner {
    let entries: [(u8, Box<dyn Solution>); _] = [
        (6, Box::new(day06::Solution06 {})),
        // aoc:solutions
    ];

    AocRunner {
        year: 2024,
        solutions: HashMap::from(entries),
    }
}

fn main() {
    let runner = create_runner();
    runner.parse_and_run();
}
