use aoc_lib::solution::Solution;
use aoc_lib::specific::intcode::Program;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::{iproduct, Itertools};

pub struct Solution19;
impl Solution19 {
    fn is_pulled(intcode: &str, x: i128, y: i128) -> bool {
        let mut drone = Program::init(intcode);
        drone.input.push_back(x);
        drone.input.push_back(y);
        drone.execute();
        drone.output[0] != 0
    }
}

impl Solution for Solution19 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        if _is_sample {
            return None;
        }

        let intcode = input.string();
        let tractor = iproduct!(0..50, 0..50)
            .filter(|(x, y)| Self::is_pulled(&intcode, *x, *y))
            .count();

        tractor.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        if _is_sample {
            return None;
        }

        let intcode = input.string();

        // Find first row in which the tractor beam is visible
        let (mut start_x, mut end_x) = (i128::MAX, i128::MIN);
        let mut start_y = 0;
        for y in 1.. {
            let beam = (0..10).filter(|x| Self::is_pulled(&intcode, *x, y)).collect_vec();
            if !beam.is_empty() {
                start_x = beam[0];
                end_x = beam[beam.len() - 1];
                start_y = y + 1;
                break;
            }
        }

        // Go row by row, update edges of beam, and check if spacecraft would fit
        let mut beam_ends = vec![0; start_y as usize];
        for y in start_y.. {
            // Update start of beam
            while !Self::is_pulled(&intcode, start_x, y) {
                start_x += 1;
            }

            // Update end of beam
            end_x = end_x.max(start_x);
            while Self::is_pulled(&intcode, end_x, y) {
                end_x += 1;
            }
            end_x -= 1;
            beam_ends.push(end_x);

            // Check if ship fits
            if y >= 99 && start_x + 99 <= beam_ends[y as usize - 99] {
                return (start_x * 10000 + (y - 99)).into_some();
            }
        }

        unreachable!()
    }
}
