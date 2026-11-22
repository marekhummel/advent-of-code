use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::prelude::types::{ProblemInput, ProblemResult};

pub fn part1(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let pipes = input.grid();
    let (letters, _) = follow_pipes(&pipes);

    letters.into()
}

pub fn part2(input: ProblemInput, _is_sample: bool) -> ProblemResult {
    let pipes = input.grid();
    let (_, steps) = follow_pipes(&pipes);

    steps.into()
}

fn follow_pipes(pipes: &Grid<char>) -> (String, u32) {
    let start_i = pipes.rows[0].iter().position(|c| *c == '|').unwrap();

    let mut letters = Vec::new();
    let mut steps = 0;

    let mut pos = Index::new(start_i, 0);
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
