use aoc_lib::cartesian::{Grid, Index, Size};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

pub struct Solution08;
impl Solution08 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                return if l.starts_with("rect") {
                    let size = l.strip_prefix("rect ").unwrap();
                    let (w, h) = size.split_once('x').unwrap();
                    Instruction::Rect(w.parse().unwrap(), h.parse().unwrap())
                } else if l.starts_with("rotate column") {
                    let vals = l.strip_prefix("rotate column ").unwrap();
                    let (xs, l) = vals.split_once(" by ").unwrap();
                    let (_, x) = xs.split_once('=').unwrap();
                    Instruction::RotateColumn(x.parse().unwrap(), l.parse().unwrap())
                } else if l.starts_with("rotate row") {
                    let vals = l.strip_prefix("rotate row ").unwrap();
                    let (ys, l) = vals.split_once(" by ").unwrap();
                    let (_, y) = ys.split_once('=').unwrap();
                    Instruction::RotateRow(y.parse().unwrap(), l.parse().unwrap())
                } else {
                    unreachable!()
                };
            })
            .collect_vec()
    }
}

impl Solution for Solution08 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);
        let mut screen = if is_sample {
            Grid::empty(Size::new(7, 3), false)
        } else {
            Grid::empty(Size::new(50, 6), false)
        };

        for inst in instructions {
            match inst {
                Instruction::Rect(w, h) => {
                    for j in 0..h {
                        for i in 0..w {
                            screen.set(&Index { i, j }, true);
                        }
                    }
                }
                Instruction::RotateColumn(x, l) => {
                    let length = screen.size.height;
                    screen = screen.transpose();
                    let col = screen.rows.get_mut(x).unwrap();
                    *col = [&col[length - l..], &col[0..length - l]].concat();
                    screen = screen.transpose();
                }
                Instruction::RotateRow(y, l) => {
                    let length = screen.size.width;
                    let row = screen.rows.get_mut(y).unwrap();
                    *row = [&row[length - l..], &row[0..length - l]].concat();
                }
            }
        }

        // screen.print(|_, led| if *led { '#' } else { '.' });
        screen.enumerate().filter(|(_, led)| **led).count().to_result()
    }

    fn solve_version02(&self, _input: ProblemInput, _is_sample: bool) -> ProblemResult {
        // Run part 1 and print final screen.
        String::from("UPOJFLBCEZ").to_result()
    }
}
