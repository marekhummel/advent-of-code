use aoc_lib::cartesian::{Direction, Position};
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
pub struct Solution18;

impl Solution18 {
    fn parse(input: ProblemInput) -> Vec<(Direction, usize)> {
        input
            .lines()
            .iter()
            .map(|line| {
                let (ds, ls, _) = line.split_whitespace().collect_tuple().unwrap();

                let dir = ds.try_into().unwrap();
                let len = ls.parse().unwrap();
                (dir, len)
            })
            .collect_vec()
    }

    fn parse2(input: ProblemInput) -> Vec<(Direction, usize)> {
        input
            .lines()
            .iter()
            .map(|line| {
                let (_, _, cs) = line.split_whitespace().collect_tuple().unwrap();
                let (ls, dc) = cs
                    .strip_prefix("(#")
                    .and_then(|s| s.strip_suffix(')'))
                    .map(|s| (&s[0..s.len() - 1], s.chars().last().unwrap()))
                    .unwrap();

                let dir = match dc {
                    '0' => Direction::East,
                    '1' => Direction::South,
                    '2' => Direction::West,
                    '3' => Direction::North,
                    _ => unreachable!(),
                };
                let len = usize::from_str_radix(ls, 16).unwrap();
                (dir, len)
            })
            .collect_vec()
    }

    fn find_vertices(edges: &[(Direction, usize)]) -> Vec<Position> {
        let mut vertex = Position::zero();
        let mut vertices = Vec::from([vertex]);
        for (dir, count) in edges {
            vertex = vertex.advance_by(*dir, *count as i128);
            vertices.push(vertex);
        }
        vertices
    }

    fn compute_area(vertices: &[Position], edges: &[(Direction, usize)]) -> u128 {
        let boundary = edges.iter().fold(0, |acc, (_, l)| acc + *l as u128);

        // Shoelace formula
        let mut area_inside = 0i128;
        for (v, nv) in vertices.iter().zip(vertices.iter().skip(1)) {
            area_inside += (v.y + nv.y) * (v.x - nv.x);
        }
        area_inside /= 2;

        // Pick's theorem
        let inside = area_inside as u128 - boundary / 2 + 1;
        boundary + inside
    }
}

impl Solution for Solution18 {
    fn solve_version01(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let edges = Self::parse(input);
        let vertices = Self::find_vertices(&edges);
        Self::compute_area(&vertices, &edges).to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        let edges = Self::parse2(input);
        let vertices = Self::find_vertices(&edges);
        Self::compute_area(&vertices, &edges).to_result()
    }
}
