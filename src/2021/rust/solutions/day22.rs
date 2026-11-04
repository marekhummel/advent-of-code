use std::collections::HashMap;

use aoc_lib::algebra::Vec3;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;

type Instruction = (bool, Vec3<i32>, Vec3<i32>);

pub struct Solution22;
impl Solution22 {
    fn parse(input: ProblemInput) -> Vec<Instruction> {
        input
            .lines()
            .iter()
            .map(|l| {
                let (action, cuboid_str) = l.split_once(' ').unwrap();
                let (x, y, z) = cuboid_str
                    .split(',')
                    .map(|rng| {
                        let (_, vals) = rng.split_once('=').unwrap();
                        let (low, high) = vals.split_once("..").unwrap();
                        (low.parse::<i32>().unwrap(), high.parse::<i32>().unwrap())
                    })
                    .collect_tuple()
                    .unwrap();
                let min = Vec3::new(x.0, y.0, z.0);
                let max = Vec3::new(x.1 + 1, y.1 + 1, z.1 + 1);
                (action == "on", min, max)
            })
            .collect()
    }

    fn get_cuboid_indices(insts: &[Instruction]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
        // Reduce problem down to the mentioned indices
        let xx = insts.iter().flat_map(|(_, s, e)| [s.x, e.x]).unique().sorted();
        let yy = insts.iter().flat_map(|(_, s, e)| [s.y, e.y]).unique().sorted();
        let zz = insts.iter().flat_map(|(_, s, e)| [s.z, e.z]).unique().sorted();

        (xx.collect(), yy.collect(), zz.collect())
    }

    fn apply_instructions(instructions: &[Instruction], xx: &[i32], yy: &[i32], zz: &[i32]) -> Vec<bool> {
        // Maps actual grid index to cuboid index
        let xx_map: HashMap<_, _> = xx.iter().enumerate().map(|(ex, x)| (x, ex)).collect();
        let yy_map: HashMap<_, _> = yy.iter().enumerate().map(|(ey, y)| (y, ey)).collect();
        let zz_map: HashMap<_, _> = zz.iter().enumerate().map(|(ez, z)| (z, ez)).collect();

        // Sets lights, but in grid which tracks cuboids (use flat vec for speed)
        let xlen = xx.len() - 1;
        let ylen = yy.len() - 1;
        let zlen = zz.len() - 1;
        let mut grid = vec![false; xlen * ylen * zlen];
        for (action, start, end) in instructions {
            for cz in zz_map[&start.z]..zz_map[&end.z] {
                for cy in yy_map[&start.y]..yy_map[&end.y] {
                    for cx in xx_map[&start.x]..xx_map[&end.x] {
                        grid[cz * xlen * ylen + cy * xlen + cx] = *action
                    }
                }
            }
        }

        grid
    }

    fn compute_lights(grid: &[bool], xx: &[i32], yy: &[i32], zz: &[i32], border: i32) -> i64 {
        // Dimensions of grid
        let xlen = xx.len() - 1;
        let ylen = yy.len() - 1;
        let zlen = zz.len() - 1;

        // Side lengths of cuboids in each dim
        let zz_pairs = zz.iter().map(|&z| z.clamp(-border, border + 1)).tuple_windows();
        let zwidths = zz_pairs.map(|(a, b)| (b - a) as i64).collect_vec();
        let yy_pairs = yy.iter().map(|&y| y.clamp(-border, border + 1)).tuple_windows();
        let ywidths = yy_pairs.map(|(a, b)| (b - a) as i64).collect_vec();
        let xx_pairs = xx.iter().map(|&x| x.clamp(-border, border + 1)).tuple_windows();
        let xwidths = xx_pairs.map(|(a, b)| (b - a) as i64).collect_vec();

        // Iterate through cuboid grid and append volume of cuboid if it is lit
        let mut total_lit = 0;
        for cz in 0..zlen {
            let z_width = zwidths[cz];
            if z_width == 0 {
                continue;
            }
            for cy in 0..ylen {
                let y_width = ywidths[cy];
                if y_width == 0 {
                    continue;
                }
                for cx in 0..xlen {
                    if grid[cz * xlen * ylen + cy * xlen + cx] {
                        let x_width = xwidths[cx];
                        let cuboid_size = z_width * y_width * x_width;
                        total_lit += cuboid_size;
                    }
                }
            }
        }
        total_lit
    }
}

impl Solution for Solution22 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::I64(590784),
            ProblemResult::I64(576028),
            ProblemResult::I64(2758514936282235),
            ProblemResult::I64(1387966280636636),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);

        let (xx, yy, zz) = Self::get_cuboid_indices(&instructions);
        let grid = Self::apply_instructions(&instructions, &xx, &yy, &zz);
        let total_lit = Self::compute_lights(&grid, &xx, &yy, &zz, 50);

        total_lit.to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let instructions = Self::parse(input);

        let (xx, yy, zz) = Self::get_cuboid_indices(&instructions);
        let grid = Self::apply_instructions(&instructions, &xx, &yy, &zz);
        let total_lit = Self::compute_lights(&grid, &xx, &yy, &zz, i32::MAX - 1);

        total_lit.to_result()
    }
}
