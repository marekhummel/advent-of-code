use std::collections::HashSet;

use aoc_lib::algebra::Vec3;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Particle {
    id: usize,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

pub struct Solution20;
impl Solution20 {
    fn parse(input: ProblemInput) -> Vec<Particle> {
        input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(id, l)| {
                let (pos, vel, acc) = l
                    .split(", ")
                    .map(|vec_str| {
                        let (_, vec) = vec_str.split_once('=').unwrap();
                        let vec_val = vec.strip_prefix('<').unwrap().strip_suffix('>').unwrap();
                        let (x, y, z) = vec_val
                            .split(',')
                            .map(|c| c.trim().parse().unwrap())
                            .collect_tuple()
                            .unwrap();
                        Vec3::new(x, y, z)
                    })
                    .collect_tuple()
                    .unwrap();
                Particle { id, pos, vel, acc }
            })
            .collect_vec()
    }
}

impl Solution for Solution20 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(0),
            ProblemResult::USize(161),
            ProblemResult::USize(1),
            ProblemResult::USize(438),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let particles = Self::parse(input);

        // Just works due to lucky input I guess
        particles
            .into_iter()
            .min_by_key(|p| (p.acc.length(), p.vel.length(), p.pos.length()))
            .unwrap()
            .id
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut particles = Self::parse(input);

        let mut last_collision = 0;
        for i in 0.. {
            for p in particles.iter_mut() {
                p.update();
            }

            let collision_pos: HashSet<_> = particles
                .iter()
                .group_by(|p| p.pos)
                .into_iter()
                .map(|(pos, grp)| (pos, grp.count()))
                .filter(|(_, grp)| *grp > 1)
                .map(|(pos, _)| pos)
                .collect();

            if !collision_pos.is_empty() {
                last_collision = i;
            }

            // Completely arbitrary sentinel
            if i - last_collision > 100 {
                break;
            }

            particles.retain(|p| !collision_pos.contains(&p.pos));
        }

        particles.len().to_result()
    }
}
