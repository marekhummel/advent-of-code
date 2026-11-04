use std::collections::{HashSet, VecDeque};

use aoc_lib::algebra::{Matrix, Vec3};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{iproduct, izip, Itertools};

pub struct Solution19;
impl Solution19 {
    fn parse(input: ProblemInput) -> Vec<Vec<Vec3<i32>>> {
        input
            .lines()
            .split(|l| l.is_empty())
            .map(|scanner| {
                scanner
                    .iter()
                    .skip(1)
                    .map(|l| {
                        let (x, y, z) = l.split(',').parsed().collect_tuple().unwrap();
                        Vec3::new(x, y, z)
                    })
                    .collect()
            })
            .collect()
    }

    fn rotations() -> Vec<Matrix<i32>> {
        // Lists all possible orientations. Technically there are 48,
        // but half of them are just the left-handed version of the other (det < 0).
        let mut rotations = Vec::new();
        for xyz in (0..3).permutations(3) {
            for (vx, vy, vz) in iproduct!([-1, 1], [-1, 1], [-1, 1]) {
                let vals = vec![vx, vy, vz];
                let mut matrix = Matrix::new(vec![vec![0; 3]; 3]);
                for (j, i, val) in izip!((0..3), &xyz, vals) {
                    matrix.values[j][*i] = val;
                }
                if matrix.det::<i32>() > 0 {
                    rotations.push(matrix);
                }
            }
        }
        rotations
    }

    fn find_absolute_locations(scanner_beacons: Vec<Vec<Vec3<i32>>>) -> (Vec<Vec3<i32>>, Vec<Vec3<i32>>) {
        let rotations = Self::rotations();

        // Start with first scanner as absolute, try to match other scanners
        let mut scanners = vec![Vec3::new(0, 0, 0)];
        let mut absolute: HashSet<_> = scanner_beacons[0].iter().copied().collect();
        let mut open: VecDeque<_> = scanner_beacons.into_iter().skip(1).collect();

        // Pick random scanner and try every orientation for it
        while let Some(scanner) = open.pop_front() {
            let mut found = false;
            for rotation in &rotations {
                // Rotate scanner and thus the relative beacon locations
                let beacons = scanner.iter().map(|&b| rotation * b).collect_vec();

                // If p and q describe the relative vectors from two scanners to the same beacon, then
                // p - q is the vector between the scanners.
                let potential_locs = iproduct!(&absolute, &beacons).map(|(&p, &q)| p - q);
                let (loc, occ) = potential_locs.counts().into_iter().max_by_key(|(_, c)| *c).unwrap();
                if occ >= 12 {
                    // Transform relative beacon locations to absolutes, because we found 12 matching beacons
                    absolute.extend(beacons.into_iter().map(|b| loc + b));
                    scanners.push(loc);
                    found = true;
                    break;
                }
            }

            // Try this scanner later, when list of known absolute beacons became bigger (assert that will happen).
            if !found {
                open.push_back(scanner);
            }
        }

        (scanners, absolute.into_iter().collect())
    }
}

impl Solution for Solution19 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(79),
            ProblemResult::USize(372),
            ProblemResult::I32(3621),
            ProblemResult::I32(12241),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let scanner_beacons = Self::parse(input);
        let (_, beacons) = Self::find_absolute_locations(scanner_beacons);
        beacons.len().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let scanner_beacons = Self::parse(input);
        let (scanners, _) = Self::find_absolute_locations(scanner_beacons);

        let scanner_pairs = scanners.into_iter().tuple_combinations();
        let max_dist = scanner_pairs.map(|(a, b)| (a - b).length()).max().unwrap();
        max_dist.to_result()
    }
}
