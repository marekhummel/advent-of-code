use aoc_lib::cartesian::{Grid, Index};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Board = Grid<Option<u8>>;

pub struct Solution04;
impl Solution04 {
    fn parse(input: ProblemInput) -> (Vec<u8>, Vec<Board>) {
        let lines = input.lines();
        let groups = lines.split(|l| l.is_empty()).collect_vec();
        let (number_str, board_strs) = groups.split_first().unwrap();

        let numbers = number_str.join("").split(',').parsed().collect();
        let boards = board_strs
            .iter()
            .map(|board_str| {
                Grid::new(
                    board_str
                        .iter()
                        .map(|row| row.split_ascii_whitespace().map(|n| Some(n.parse().unwrap())).collect())
                        .collect(),
                )
            })
            .collect();

        (numbers, boards)
    }

    fn update_board(board: &mut Board, number: u8) {
        let tile = board.enumerate().find(|(_, n)| **n == Some(number));
        if let Some((idx, _)) = tile {
            board.set(&idx, None);
        }
    }

    fn is_won(board: &Board) -> bool {
        let winning_row = (0..5).any(|j| (0..5).all(|i| board.get(&Index { i, j }).is_none()));
        let winning_col = (0..5).any(|i| (0..5).all(|j| board.get(&Index { i, j }).is_none()));
        winning_row || winning_col
    }

    fn score(board: &Board, number: u8) -> u32 {
        let unmarked: u32 = board.iter().flatten().copied().map_into::<u32>().sum();
        unmarked * number as u32
    }
}

impl Solution for Solution04 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U32(4512),
            ProblemResult::U32(22680),
            ProblemResult::U32(1924),
            ProblemResult::U32(16168),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (numbers, mut boards) = Self::parse(input);

        for number in numbers {
            for board in &mut boards {
                Self::update_board(board, number);
                if Self::is_won(board) {
                    return Self::score(board, number).to_result();
                }
            }
        }

        unreachable!()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let (numbers, mut boards) = Self::parse(input);

        for number in numbers {
            let mut delete = Vec::new();
            let remaining = boards.len();
            for board in &mut boards.iter_mut() {
                Self::update_board(board, number);
                if Self::is_won(board) {
                    if remaining == 1 {
                        return Self::score(board, number).to_result();
                    }

                    delete.push(board.clone());
                }
            }

            boards.retain(|b| !delete.contains(b));
        }

        unreachable!()
    }
}
