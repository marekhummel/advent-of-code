use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};

pub struct Solution12;
impl Solution12 {}

impl Solution for Solution12 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I128(25),
            ProblemResult::I128(1133),
            ProblemResult::I128(286),
            ProblemResult::I128(61053),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut direction = Direction::East;
        let mut position = Position::zero();

        for line in input.lines() {
            let (action, value_str) = line.split_at(1);
            let value: i128 = value_str.parse().unwrap();

            match action {
                "N" | "E" | "S" | "W" => position = position.advance_by(action.try_into().unwrap(), value),
                "L" if value == 90 => direction = direction.left(),
                "L" if value == 180 => direction = direction.inverse(),
                "L" if value == 270 => direction = direction.right(),
                "R" if value == 90 => direction = direction.right(),
                "R" if value == 180 => direction = direction.inverse(),
                "R" if value == 270 => direction = direction.left(),
                "F" => position = position.advance_by(direction, value),
                _ => unreachable!(),
            }
        }

        (position.x.abs() + position.y.abs()).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut ship = Position::zero();
        let mut waypoint = Position::new(10, -1); // relative to ship

        for line in input.lines() {
            let (action, value_str) = line.split_at(1);
            let value: i128 = value_str.parse().unwrap();

            match action {
                "N" | "E" | "S" | "W" => waypoint = waypoint.advance_by(action.try_into().unwrap(), value),
                "F" => ship = Position::new(ship.x + waypoint.x * value, ship.y + waypoint.y * value),
                "L" | "R" => {
                    // Note that L and R are essentially swapped, cause we use North as negative coordinates.
                    waypoint = match (action, value) {
                        ("L", 90) | ("R", 270) => Position::new(waypoint.y, -waypoint.x),
                        ("L", 180) | ("R", 180) => Position::new(-waypoint.x, -waypoint.y),
                        ("L", 270) | ("R", 90) => Position::new(-waypoint.y, waypoint.x),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        (ship.x.abs() + ship.y.abs()).to_result()
    }
}
