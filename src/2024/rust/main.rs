mod solutions;

use std::collections::HashMap;

use aoc_lib::prelude::runner::AocRunner;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::solution;

use solutions::day06;
// aoc:imports

fn create_runner() -> AocRunner {
    let entries: [(u8, Solution); _] = [
        (6, solution!(day06)),
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
