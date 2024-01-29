use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution21;
impl Solution21 {
    fn run() -> Vec<usize> {
        // Program in input resembles the following. Register #0 is used for the break, #5 for the value for comparison,
        // #1 and #4 are temps, #2 for the inner loop, #3 is the PC. Note that the instruction C /= 256 is implemented
        // as a loop as well.
        //
        // LOOP {
        //     C = F | 65536
        //     F = 10362650
        //
        //     LOOP {
        //         F = (((F + (C & 255)) & 16777215) * 65899) & 16777215
        //         BREAK IF (256 > C)
        //         C /= 256
        //     }
        //     BREAK IF (F == A)
        // }

        let mut fs = Vec::new();

        let mut f = 0;
        let mut c;
        loop {
            c = f | 65536;
            f = 10362650;
            loop {
                f = (((f + (c & 255)) & 16777215) * 65899) & 16777215;
                if 256 > c {
                    break;
                }
                c /= 256;
            }

            if fs.contains(&f) {
                return fs;
            }

            fs.push(f);
        }
    }
}

impl Solution for Solution21 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let fs = Self::run();
        fs[0].into_some()
    }

    fn solve_version02(&self, input: ProblemInput, is_sample: bool) -> Option<ProblemResult> {
        if is_sample {
            return None;
        }

        let fs = Self::run();
        fs.into_iter().last().unwrap().into_some()
    }
}
