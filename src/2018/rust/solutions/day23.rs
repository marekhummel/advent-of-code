use std::cmp::Reverse;
use std::collections::BinaryHeap;

use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::{iproduct, Itertools};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn len(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }

    fn mandist(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

struct Nanobot {
    pos: Pos,
    rad: u32,
}

type Box = (Pos, Pos);

pub struct Solution23;
impl Solution23 {
    fn parse(input: ProblemInput) -> Vec<Nanobot> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (pos_str, rad_str) = l.split_once(", ").unwrap();
                let (x, y, z) = pos_str
                    .trim_start_matches("pos=<")
                    .trim_end_matches('>')
                    .split(',')
                    .collect_tuple()
                    .unwrap();
                let rad = rad_str.trim_start_matches("r=").parse().unwrap();

                Nanobot {
                    pos: Pos {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                        z: z.parse().unwrap(),
                    },
                    rad,
                }
            })
            .collect()
    }

    fn does_intersect(bbox: &Box, bot: &Nanobot) -> bool {
        let (low, high) = bbox;

        let mut d = 0;
        d += bot.pos.mandist(low) as i32;
        d += bot.pos.mandist(high) as i32;
        d -= high.x - low.x;
        d -= high.y - low.y;
        d -= high.z - low.z;

        d <= bot.rad as i32 * 2
    }
}

impl Solution for Solution23 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(7),
            ProblemResult::USize(417),
            ProblemResult::U32(36),
            ProblemResult::U32(112997634),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let nanobots = Self::parse(input);

        let strongest = nanobots.iter().max_by_key(|nb| nb.rad).unwrap();
        let in_range = nanobots
            .iter()
            .filter(|nb| strongest.pos.mandist(&nb.pos) <= strongest.rad)
            .count();

        in_range.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        // https://www.reddit.com/r/adventofcode/comments/a8s17l/comment/ecfmpy0/
        let nanobots = Self::parse(input);
        let max_dist = nanobots
            .iter()
            .flat_map(|nb| {
                let r = nb.rad as i32;
                [
                    nb.pos.x + r,
                    nb.pos.x - r,
                    nb.pos.y + r,
                    nb.pos.y - r,
                    nb.pos.z + r,
                    nb.pos.z - r,
                ]
            })
            .map(|c| c.abs())
            .max()
            .unwrap();

        let box_size = 1 << (max_dist.ilog2() + 1);
        let initial_box = (
            Pos {
                x: -box_size,
                y: -box_size,
                z: -box_size,
            },
            Pos {
                x: box_size - 1,
                y: box_size - 1,
                z: box_size - 1,
            },
        );
        let mut workheap =
            BinaryHeap::from([(nanobots.len(), 2 * box_size, Reverse(initial_box.0.len()), initial_box)]);

        while let Some((_reach, sz, Reverse(dist_to_orig), bbox)) = workheap.pop() {
            if sz == 1 {
                return dist_to_orig.to_result();
            }

            // Split box into eight octants, each with half the side length
            let newsz = sz / 2;
            for (ox, oy, oz) in iproduct!([0, 1], [0, 1], [0, 1]) {
                let new_low = Pos {
                    x: bbox.0.x + ox * newsz,
                    y: bbox.0.y + oy * newsz,
                    z: bbox.0.z + oz * newsz,
                };

                let new_high = Pos {
                    x: new_low.x + newsz - 1,
                    y: new_low.y + newsz - 1,
                    z: new_low.z + newsz - 1,
                };

                let dist = new_low.len();
                let newbox = (new_low, new_high);
                let newreach = nanobots.iter().filter(|b| Self::does_intersect(&newbox, b)).count();
                workheap.push((newreach, newsz, Reverse(dist), newbox));
            }
        }

        unreachable!()
    }
}
