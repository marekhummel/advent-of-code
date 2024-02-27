use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

pub struct Solution06;
impl Solution06 {}

impl Solution for Solution06 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(5934),
            ProblemResult::USize(359344),
            ProblemResult::U64(26984457539),
            ProblemResult::U64(1629570219571),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        // Simple simulation (could use part two with DAYS = 80 as well)
        let lantern_fish = input.string().split(',').parsed::<u8>().collect_vec();
        let final_school = (0..80).fold(lantern_fish, |school, _| {
            school
                .into_iter()
                .flat_map(|fish| if fish == 0 { vec![6, 8] } else { vec![fish - 1] })
                .collect_vec()
        });

        final_school.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        const DAYS: usize = 256;

        // Setup growth as a map, for each day how many new fish are reproduced
        let lantern_fish = input.string().split(',').parsed::<usize>().collect_vec();
        let mut count = lantern_fish.len() as u64;
        let mut growth = [0u64; DAYS + 9];
        for fish in lantern_fish {
            growth[fish] += 1;
        }

        for day in 0..DAYS {
            count += growth[day]; // new fish
            growth[day + 7] += growth[day]; // parent fish will reproduce again in 7 days
            growth[day + 9] += growth[day]; // child fish will reproduce in 7 + 2 days
        }

        count.to_result()
    }
}
