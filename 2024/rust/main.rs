mod solutions;

use aoc_lib::prelude::runner::AocRunner;
use aoc_lib::prelude::solution::Solution;

use std::env;

const ALL: bool = true;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

fn create_runner() -> AocRunner {
    let solutions: Vec<Box<dyn Solution>> = vec![
        //
    ];

    AocRunner { year: 2024, solutions }
}

fn main() {
    let arg = env::args().nth(1);
    let runner = create_runner();
    runner.run(arg, ALL, VERSION, USE_SAMPLE);
}

#[cfg(test)]
mod tests2024 {
    use aoc_lib::test_day;

    //
}
