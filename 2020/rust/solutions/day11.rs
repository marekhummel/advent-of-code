use aoc_lib::cartesian::{Direction, Grid, Index};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution11;
impl Solution11 {
    fn take_seats<F>(mut seats: Grid<char>, count_occupied: F, threshold_occ: usize) -> Grid<char>
    where
        F: Fn(&Grid<char>, Index) -> usize,
    {
        let mut stabilized = false;
        while !stabilized {
            stabilized = true;
            let mut new_seats = Grid::empty(seats.size, '.');
            for (idx, seat) in seats.enumerate() {
                let adjacent_occ = count_occupied(&seats, idx);

                let new_state = match seat {
                    'L' if adjacent_occ == 0 => '#',
                    'L' => 'L',
                    '#' if adjacent_occ >= threshold_occ => 'L',
                    '#' => '#',
                    '.' => '.',
                    _ => unreachable!(),
                };

                stabilized &= &new_state == seat;
                new_seats.set(&idx, new_state);
            }

            seats = new_seats
        }

        seats
    }

    fn count_visible_occupied(seats: &Grid<char>, start: Index) -> usize {
        let directions = [
            vec![Direction::North],
            vec![Direction::East],
            vec![Direction::South],
            vec![Direction::West],
            vec![Direction::North, Direction::East],
            vec![Direction::North, Direction::West],
            vec![Direction::South, Direction::East],
            vec![Direction::South, Direction::West],
        ];

        let mut occupied = 0;
        for dirs in directions {
            let mut outlook = Some(start);
            loop {
                outlook = dirs
                    .iter()
                    .fold(outlook, |idx, dir| idx.and_then(|i| i.advance_check(*dir, seats.size)));

                match outlook.map(|o| seats.get(&o)) {
                    Some('.') => continue,
                    Some('#') => occupied += 1,
                    Some('L') => (),
                    None => (),
                    _ => unreachable!(),
                }
                break;
            }
        }

        occupied
    }
}

impl Solution for Solution11 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
            ProblemResult::Unsolved,
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let seating_area = input.grid();

        let count_occupied = |seats: &Grid<char>, idx: Index| {
            idx.moore_neighbors(seats.size)
                .into_iter()
                .map(|nb| seats.get(&nb))
                .filter(|adj| **adj == '#')
                .count()
        };

        let final_seating = Self::take_seats(seating_area, count_occupied, 4);
        final_seating.iter().filter(|seat| **seat == '#').count().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let seating_area = input.grid();
        let final_seating = Self::take_seats(seating_area, Self::count_visible_occupied, 5);
        final_seating.iter().filter(|seat| **seat == '#').count().to_result()
    }
}
