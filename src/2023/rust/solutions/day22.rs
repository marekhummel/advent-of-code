use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
struct Position3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Position3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0},{1},{2}", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    id: usize,
    from: Position3,
    to: Position3,
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

impl Brick {
    fn lower(&mut self) {
        self.from = Position3 {
            x: self.from.x,
            y: self.from.y,
            z: self.from.z - 1,
        };
        self.to = Position3 {
            x: self.to.x,
            y: self.to.y,
            z: self.to.z - 1,
        };
    }

    fn on_ground(&self) -> bool {
        self.from.z == 1
    }
}

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> Vec<Brick> {
        input
            .lines()
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                let (start, end) = line.split_once('~').unwrap();
                let (xs, ys, zs) = start.split(',').parsed::<usize>().collect_tuple().unwrap();
                let (xe, ye, ze) = end.split(',').parsed::<usize>().collect_tuple().unwrap();

                let (x1, x2) = (xs.min(xe), xs.max(xe));
                let (y1, y2) = (ys.min(ye), ys.max(ye));
                let (z1, z2) = (zs.min(ze), zs.max(ze));

                Brick {
                    id: i,
                    from: Position3 { x: x1, y: y1, z: z1 },
                    to: Position3 { x: x2, y: y2, z: z2 },
                    supported_by: Vec::new(),
                    supports: Vec::new(),
                }
            })
            .sorted_by_key(|b| b.from.z)
            .collect_vec()
    }

    fn let_bricks_fall(bricks: &mut [Brick]) {
        // Easy lookup which groups bricks by the z-level they stop at
        let mut landed_bricks: HashMap<usize, Vec<Brick>> = HashMap::new();
        let mut support_lookup: HashMap<usize, Vec<usize>> = HashMap::new();

        for fb in bricks.iter_mut() {
            while !fb.on_ground() {
                if let Some(lbs) = landed_bricks.get(&(fb.from.z - 1)) {
                    for lb in lbs {
                        if (fb.from.x <= lb.to.x && lb.from.x <= fb.to.x)
                            && (fb.from.y <= lb.to.y && lb.from.y <= fb.to.y)
                        {
                            fb.supported_by.push(lb.id);
                            support_lookup.entry(lb.id).or_default().push(fb.id);
                        }
                    }

                    if !fb.supported_by.is_empty() {
                        break;
                    }
                }

                fb.lower();
            }

            landed_bricks.entry(fb.to.z).or_default().push(fb.clone());
        }

        // Double sided lookup in brick, second direction has to be done afterwards
        for b in bricks.iter_mut() {
            b.supports.extend(support_lookup.get(&b.id).unwrap_or(&Vec::new()));
        }
    }

    fn compute_disintegration_chain(bricks: &[Brick]) -> HashMap<usize, u32> {
        let brick_lookup = bricks
            .iter()
            .map(|b| (b.id, b.clone()))
            .collect::<HashMap<usize, Brick>>();

        let mut disintegration = HashMap::new();
        for brick in bricks.iter() {
            let mut chain = HashSet::from([brick.id]);
            let mut to_disintegrate = BinaryHeap::from([(-(brick.to.z as i32), brick.id)]);
            while let Some((_, c)) = to_disintegrate.pop() {
                let current_brick = brick_lookup.get(&c).unwrap();
                for ab in current_brick.supports.iter() {
                    let above_brick = brick_lookup.get(ab).unwrap();
                    if above_brick.supported_by.iter().all(|sb| chain.contains(sb)) {
                        chain.insert(above_brick.id);
                        to_disintegrate.push((-(above_brick.to.z as i32), above_brick.id));
                    }
                }
            }
            disintegration.insert(brick.id, chain);
        }

        // Turn chain into chain length and subtract own brick from chain length
        disintegration
            .into_iter()
            .map(|(id, chain)| (id, chain.len() as u32 - 1))
            .collect()
    }
}

impl Solution for Solution22 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut bricks = Self::parse(input);
        Self::let_bricks_fall(&mut bricks);
        let disintegration = Self::compute_disintegration_chain(&bricks);

        // Bricks that can be disintegrated have chain length of zero
        disintegration.values().filter(|chain| **chain == 0).count().to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let mut bricks = Self::parse(input);
        Self::let_bricks_fall(&mut bricks);
        let disintegration = Self::compute_disintegration_chain(&bricks);

        disintegration.values().sum::<u32>().to_result()
    }
}
