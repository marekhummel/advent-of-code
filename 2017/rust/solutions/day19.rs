use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};

pub struct Solution19;
impl Solution19 {
    fn follow_pipes(pipes: &Grid<char>) -> (String, u32) {
        let start_i = pipes.rows[0].iter().position(|c| *c == '|').unwrap();

        let mut letters = Vec::new();
        let mut steps = 0;

        let mut pos = Index { i: start_i, j: 0 };
        let mut dir = Direction::South;
        loop {
            match pipes.get(&pos) {
                ' ' => break,
                '+' => {
                    for turn in dir.turn() {
                        let next_pos = pos.advance(turn);
                        if *pipes.get(&next_pos) != ' ' {
                            dir = turn;
                            pos = next_pos;
                            break;
                        }
                    }
                }
                l => {
                    pos = pos.advance(dir);
                    if *l != '-' && *l != '|' {
                        letters.push(*l);
                    }
                }
            }
            steps += 1;
        }

        (letters.into_iter().collect::<String>(), steps)
    }
}

impl Solution for Solution19 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let pipes = input.grid();
        let (letters, _) = Self::follow_pipes(&pipes);

        letters.into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let pipes = input.grid();
        let (_, steps) = Self::follow_pipes(&pipes);

        steps.into_some()
    }
}