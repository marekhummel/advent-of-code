mod solutions;

use aoc_lib::prelude::runner::AocRunner;
use std::env;

use aoc_lib::prelude::solution::Solution;

const ALL: bool = true;
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
