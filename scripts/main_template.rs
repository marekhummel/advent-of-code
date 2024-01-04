mod solutions;

use aoc_lib::runner::AocRunner;
use std::env;

use aoc_lib::solution::Solution;

const ALL: bool = false;
const VERSION: u8 = 1;
const USE_SAMPLE: bool = true;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![
        //
    ];

    let arg = env::args().nth(1);
    let runner = AocRunner { year: XXXX, solutions };
    runner.run(arg, ALL, VERSION, USE_SAMPLE);
}
