use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_lib::cartesian::{Index, Size};
use aoc_lib::solution::Solution;
use aoc_lib::types::{IntoSome, ProblemInput, ProblemResult};
use itertools::iproduct;

#[derive(Debug, PartialEq, Eq)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Gear {
    Nothing,
    Torch,
    Climbing,
}

struct Cave {
    depth: usize,
    target: Index,
    geologic: HashMap<Index, usize>,
    erosion: HashMap<Index, usize>,
}

impl Cave {
    fn geologic_index(&mut self, idx: Index) -> usize {
        if let Some(geo_index) = self.geologic.get(&idx) {
            return *geo_index;
        }

        let geologic_index = match (idx.i, idx.j) {
            (0, 0) => 0,
            (i, j) if i == self.target.i && j == self.target.j => 0,
            (i, 0) => i * 16807,
            (0, j) => j * 48271,
            (i, j) => self.erosion_level(Index { i, j: j - 1 }) * self.erosion_level(Index { i: i - 1, j }),
        };
        self.geologic.insert(idx, geologic_index);
        geologic_index
    }

    fn erosion_level(&mut self, idx: Index) -> usize {
        if let Some(erosion_lvl) = self.erosion.get(&idx) {
            return *erosion_lvl;
        }

        let geologic_index = self.geologic_index(idx);
        let erosion_level = (geologic_index + self.depth) % 20183;
        self.erosion.insert(idx, erosion_level);
        erosion_level
    }

    fn region(&mut self, idx: Index) -> Region {
        match self.erosion_level(idx) % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }
}

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> (usize, Index) {
        let lines = input.lines();

        let depth = lines[0].split_once(' ').unwrap().1.parse().unwrap();
        let (i, j) = lines[1].split_once(' ').unwrap().1.split_once(',').unwrap();
        let target = Index {
            i: i.parse().unwrap(),
            j: j.parse().unwrap(),
        };

        (depth, target)
    }
}

impl Solution for Solution22 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (depth, target) = Self::parse(input);

        let mut cave = Cave {
            depth,
            target,
            geologic: HashMap::new(),
            erosion: HashMap::new(),
        };

        iproduct!(0..=target.i, 0..=target.j)
            .map(|(i, j)| cave.erosion_level(Index { i, j }) % 3)
            .sum::<usize>()
            .into_some()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> Option<ProblemResult> {
        let (depth, target) = Self::parse(input);

        let mut cave = Cave {
            depth,
            target,
            geologic: HashMap::new(),
            erosion: HashMap::new(),
        };

        let cave_size = Size {
            width: usize::MAX,
            height: usize::MAX,
        };
        let mut queue = BinaryHeap::from([Reverse((0, Index { i: 0, j: 0 }, Gear::Torch))]);
        let mut seen = HashSet::new();

        while let Some(Reverse((time, idx, gear))) = queue.pop() {
            if idx == target && gear == Gear::Torch {
                return time.into_some();
            }

            // No need to revisit regions with the same tool
            if seen.contains(&(idx, gear)) {
                continue;
            }
            seen.insert((idx, gear));

            // Switch gears
            match (cave.region(idx), &gear) {
                (Region::Rocky, Gear::Torch) => queue.push(Reverse((time + 7, idx, Gear::Climbing))),
                (Region::Rocky, Gear::Climbing) => queue.push(Reverse((time + 7, idx, Gear::Torch))),
                (Region::Wet, Gear::Nothing) => queue.push(Reverse((time + 7, idx, Gear::Climbing))),
                (Region::Wet, Gear::Climbing) => queue.push(Reverse((time + 7, idx, Gear::Nothing))),
                (Region::Narrow, Gear::Nothing) => queue.push(Reverse((time + 7, idx, Gear::Torch))),
                (Region::Narrow, Gear::Torch) => queue.push(Reverse((time + 7, idx, Gear::Nothing))),
                (Region::Rocky, Gear::Nothing) | (Region::Wet, Gear::Torch) | (Region::Narrow, Gear::Climbing) => {
                    unreachable!()
                }
            }

            // Go to next region
            for next in idx.von_neumann_neighbors(cave_size) {
                match (cave.region(next), &gear) {
                    (Region::Rocky, Gear::Torch)
                    | (Region::Rocky, Gear::Climbing)
                    | (Region::Wet, Gear::Nothing)
                    | (Region::Wet, Gear::Climbing)
                    | (Region::Narrow, Gear::Nothing)
                    | (Region::Narrow, Gear::Torch) => queue.push(Reverse((time + 1, next, gear))),
                    (Region::Rocky, Gear::Nothing) | (Region::Wet, Gear::Torch) | (Region::Narrow, Gear::Climbing) => {}
                }
            }
        }

        None
    }
}
