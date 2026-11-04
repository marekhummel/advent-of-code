use std::collections::{HashSet, VecDeque};

use aoc_lib::algebra::Vec3;
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{iproduct, Itertools};

type Cube = Vec3<i16>;

pub struct Solution18;
impl Solution18 {
    fn parse(input: ProblemInput) -> Vec<Cube> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (x, y, z) = l.split(',').parsed().collect_tuple().unwrap();
                Vec3::new(x, y, z)
            })
            .collect()
    }

    fn deltas() -> [Cube; 6] {
        [
            Vec3::new(1, 0, 0),
            Vec3::new(-1, 0, 0),
            Vec3::new(0, 1, 0),
            Vec3::new(0, -1, 0),
            Vec3::new(0, 0, 1),
            Vec3::new(0, 0, -1),
        ]
    }

    fn count_sides(droplets: &[Cube], no_air: &HashSet<Cube>) -> u16 {
        let deltas = Self::deltas();
        let mut sides = 0;
        for droplet in droplets {
            for delta in &deltas {
                if !no_air.contains(&(*droplet + *delta)) {
                    sides += 1;
                }
            }
        }

        sides
    }
}

impl Solution for Solution18 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::U16(64),
            ProblemResult::U16(4418),
            ProblemResult::U16(58),
            ProblemResult::U16(2486),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let droplets = Self::parse(input);
        let no_air: HashSet<_> = droplets.iter().cloned().collect();
        let sides = Self::count_sides(&droplets, &no_air);
        sides.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let droplets = Self::parse(input);

        // Compute bounds
        let (x_min, x_max) = droplets.iter().map(|c| c.x).minmax().into_option().unwrap();
        let (y_min, y_max) = droplets.iter().map(|c| c.y).minmax().into_option().unwrap();
        let (z_min, z_max) = droplets.iter().map(|c| c.z).minmax().into_option().unwrap();
        let x_range = x_min..=x_max;
        let y_range = y_min..=y_max;
        let z_range = z_min..=z_max;

        // Start with droplets as no air cubes
        let mut no_air: HashSet<_> = droplets.iter().cloned().collect();
        let deltas = Self::deltas();

        for (x, y, z) in iproduct!(x_range.clone(), y_range.clone(), z_range.clone()) {
            // If current cube is no air, skip
            let center = Vec3::new(x, y, z);
            if no_air.contains(&center) {
                continue;
            }

            // Flood fill from current center
            let mut queue = VecDeque::from([center]);
            let mut pocket = HashSet::new();
            let mut is_interior = true;

            while let Some(cube) = queue.pop_front() {
                if !pocket.insert(cube) {
                    continue;
                }

                // Reached out of bounds, this is no pockets
                if !x_range.contains(&cube.x) || !y_range.contains(&cube.y) || !z_range.contains(&cube.z) {
                    is_interior = false;
                    break;
                }

                // Compute next frontiers
                for delta in &deltas {
                    let next = cube + *delta;
                    if !no_air.contains(&next) {
                        queue.push_back(next)
                    }
                }
            }

            // If filling ended because we reached a droplet on every front, we found a pocket
            if is_interior {
                no_air.extend(pocket);
            }
        }

        let sides = Self::count_sides(&droplets, &no_air);
        sides.to_result()
    }
}
