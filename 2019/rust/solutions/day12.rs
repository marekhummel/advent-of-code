use std::cell::RefCell;
use std::collections::HashSet;

use aoc_lib::math::lcm;
use aoc_lib::solution::Solution;
use aoc_lib::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn apply_gravity(&mut self, other: &mut Moon) {
        let delta_x = (other.pos.x - self.pos.x).signum();
        self.vel.x += delta_x;
        other.vel.x -= delta_x;

        let delta_y = (other.pos.y - self.pos.y).signum();
        self.vel.y += delta_y;
        other.vel.y -= delta_y;

        let delta_z = (other.pos.z - self.pos.z).signum();
        self.vel.z += delta_z;
        other.vel.z -= delta_z;
    }

    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn total_energy(&self) -> i32 {
        let pot = self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
        let kin = self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
        pot * kin
    }

    fn states(&self) -> ((i32, i32), (i32, i32), (i32, i32)) {
        (
            (self.pos.x, self.vel.x),
            (self.pos.y, self.vel.y),
            (self.pos.z, self.vel.z),
        )
    }
}

pub struct Solution12;
impl Solution12 {
    fn parse(input: ProblemInput) -> Vec<RefCell<Moon>> {
        // <x=3, y=5, z=-1>
        let rgx = Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
        input
            .lines()
            .into_iter()
            .map(|l| {
                let captures = rgx.captures(&l).unwrap();
                let x = captures.name("x").unwrap().as_str().parse().unwrap();
                let y = captures.name("y").unwrap().as_str().parse().unwrap();
                let z = captures.name("z").unwrap().as_str().parse().unwrap();

                RefCell::new(Moon {
                    pos: Vec3 { x, y, z },
                    vel: Vec3 { x: 0, y: 0, z: 0 },
                })
            })
            .collect()
    }

    fn step(moons: &[RefCell<Moon>]) {
        // Velocity (gravity)
        for (m1, m2) in moons.iter().tuple_combinations() {
            m1.borrow_mut().apply_gravity(&mut m2.borrow_mut());
        }

        // Position
        for moon in moons.iter() {
            moon.borrow_mut().apply_velocity();
        }
    }
}

impl Solution for Solution12 {
    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let moons = Self::parse(input);
        let steps = if is_sample { 10 } else { 1000 };

        for _ in 0..steps {
            Self::step(&moons);
        }

        moons
            .into_iter()
            .map(|m| m.borrow().total_energy())
            .sum::<i32>()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let moons = Self::parse(input);
        let mut states_x = HashSet::new();
        let mut states_y = HashSet::new();
        let mut states_z = HashSet::new();

        // Compute the period of each coordinate seperately, since they are not dependent
        let mut period_x = None;
        let mut period_y = None;
        let mut period_z = None;

        for step in 0.. {
            // Compute state for each dimension
            let states = moons.iter().map(|m| m.borrow().states()).collect_vec();
            let state_x = states.iter().map(|(sx, _, _)| *sx).collect_vec();
            let state_y = states.iter().map(|(_, sy, _)| *sy).collect_vec();
            let state_z = states.iter().map(|(_, _, sz)| *sz).collect_vec();

            // Check if period is found for each dimension
            if period_x.is_none() {
                if states_x.contains(&state_x) {
                    period_x = Some(step);
                }
                states_x.insert(state_x);
            }

            if period_y.is_none() {
                if states_y.contains(&state_y) {
                    period_y = Some(step);
                }
                states_y.insert(state_y);
            }

            if period_z.is_none() {
                if states_z.contains(&state_z) {
                    period_z = Some(step);
                }
                states_z.insert(state_z);
            }

            // If period has been found for all dimensions, return lcm of those
            if let (Some(px), Some(py), Some(pz)) = (period_x, period_y, period_z) {
                return lcm(&[px, py, pz]).to_result();
            }

            // Update moons
            Self::step(&moons);
        }

        unreachable!()
    }
}
