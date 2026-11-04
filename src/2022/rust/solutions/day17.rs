use std::collections::{HashMap, HashSet};

use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use num::Integer;

pub struct Solution17;
impl Solution17 {
    fn parse(input: ProblemInput) -> Vec<Direction> {
        input.string().chars().map(|c| c.try_into().unwrap()).collect()
    }

    fn rocks() -> [Vec<(i128, i128)>; 5] {
        [
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
            vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],
            vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            vec![(0, 0), (1, 0), (0, -1), (1, -1)],
        ]
    }

    fn drop_all_rocks(total_rocks: i128, jet_pattern: &[Direction]) -> i128 {
        let mut rocks = Self::rocks().into_iter().enumerate().cycle();
        let mut jets = jet_pattern.iter().enumerate().cycle().peekable();
        let mut chamber = HashSet::new();
        let mut top = 0;

        let mut seen: HashMap<(usize, usize), (i128, i128)> = HashMap::new();

        for step in 0..total_rocks {
            let (i, rock) = rocks.next().unwrap();
            let &(j, _) = jets.peek().unwrap();

            // Caching
            if let Some((prev_step, prev_top)) = seen.get(&(i, j)) {
                // Only return if extrapolating with that cycle length results on exactly the desired rock count
                let cycle = step - prev_step;
                let (cycles, remaining_rocks) = (total_rocks - step).div_rem(&cycle);
                if remaining_rocks == 0 {
                    return top + (top - prev_top) * cycles;
                }
            }
            seen.insert((i, j), (step, top));

            // Let block fall
            let mut pos = Position::new(2, top - 4);
            loop {
                // Jet
                let (_, &jet) = jets.next().unwrap();
                let mut pushed = rock.iter().map(|&b| (pos + b).advance_by(jet, 1));
                if pushed.all(|b| !chamber.contains(&b) && (0..7).contains(&b.x)) {
                    pos = pos.advance_by(jet, 1);
                };

                // Drop
                let mut fallen = rock.iter().map(|&b| (pos + b).advance_by(Direction::South, 1));
                if fallen.all(|b| !chamber.contains(&b) && b.y < 0) {
                    pos = pos.advance_by(Direction::South, 1);
                } else {
                    // Rock rests
                    chamber.extend(rock.iter().map(|&b| (pos + b)));
                    top = top.min(rock.iter().map(|&b| (pos + b).y).min().unwrap());
                    break;
                }
            }
        }

        top
    }
}

impl Solution for Solution17 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I128(3068),
            ProblemResult::I128(3181),
            ProblemResult::I128(1514285714288),
            ProblemResult::I128(1570434782634),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let jets = Self::parse(input);
        let top = Self::drop_all_rocks(2022, &jets);
        (-top).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let jets = Self::parse(input);
        let top = Self::drop_all_rocks(1_000_000_000_000, &jets);
        (-top).to_result()
    }
}
