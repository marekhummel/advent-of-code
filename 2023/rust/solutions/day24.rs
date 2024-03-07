use aoc_lib::algebra::{self, Matrix, Vec3};
use aoc_lib::iterator::ParsedExt;
use aoc_lib::prelude::solution::Solution;
use aoc_lib::prelude::types::{ProblemInput, ProblemResult, ToResult};
use itertools::Itertools;
use num::bigint::BigInt;

#[derive(Debug)]
struct Hailstone {
    pos: Vec3<i128>,
    vel: Vec3<i128>,
}

#[derive(Debug)]
enum LineRelation {
    Intersection { x: f64, y: f64, s: f64, t: f64 },
    Parallel,
    Collinear,
}

pub struct Solution24;
impl Solution24 {
    fn parse(input: ProblemInput) -> Vec<Hailstone> {
        input
            .lines()
            .into_iter()
            .map(|l| {
                let (pos_str, vel_str) = l.split_once('@').unwrap();
                let (px, py, pz) = pos_str.split(',').parsed().collect_tuple().unwrap();
                let (vx, vy, vz) = vel_str.split(',').parsed().collect_tuple().unwrap();
                Hailstone {
                    pos: Vec3::new(px, py, pz),
                    vel: Vec3::new(vx, vy, vz),
                }
            })
            .collect_vec()
    }

    fn intersecting(hail1: &Hailstone, hail2: &Hailstone) -> LineRelation {
        let (a, u) = (hail1.pos, hail1.vel);
        let (b, v) = (hail2.pos, hail2.vel);
        let uv = u.x * v.y - u.y * v.x;

        if uv == 0 {
            if (a.x - b.x) / v.x == (a.y - b.y) / v.y {
                return LineRelation::Collinear;
            } else {
                return LineRelation::Parallel;
            }
        }

        // A + s * U = B + t * V
        // U * s - V * t = B - A

        // s = (B.x - A.x + V.x * t) / U.x                                        | s
        // U.y * (B.x - A.x + V.x * t) / U.x - V.y * t = B.y - A.y
        // U.y * (B.x - A.x + V.x * t) - V.y * t * U.x = (B.y - A.y) * U.x
        // U.y * V.x * t - V.y * t * U.x = (B.y - A.y) * U.x - (B.x - A.x) * U.y
        // t * (U.y * V.x - V.y * U.x) = (B.y - A.y) * U.x - (B.x - A.x) * U.y
        // t = ((B.y - A.y) * U.x - (B.x - A.x) * U.y) / (U.y * V.x - V.y * U.x),| t
        let t = ((b.y - a.y) * u.x - (b.x - a.x) * u.y) as f64 / -uv as f64;
        let s = (b.x as f64 - a.x as f64 + v.x as f64 * t) / u.x as f64;

        LineRelation::Intersection {
            x: a.x as f64 + s * u.x as f64,
            y: a.y as f64 + s * u.y as f64,
            s,
            t,
        }
    }
}

impl Solution for Solution24 {
    fn results(&self) -> [ProblemResult; 4] {
        [
            ProblemResult::USize(2),
            ProblemResult::USize(16812),
            ProblemResult::BigInt(BigInt::from(47)),
            ProblemResult::BigInt(BigInt::from(880547248556435u64)),
        ]
    }

    fn solve_version01(&self, input: ProblemInput, is_sample: bool) -> ProblemResult {
        let range = if is_sample { 7.0..=27.0 } else { 2e14..=4e14 };
        let hailstones = Self::parse(input);

        hailstones
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                if let LineRelation::Intersection { x, y, s, t } = Self::intersecting(a, b) {
                    return s >= 0.0 && t >= 0.0 && range.contains(&x) && range.contains(&y);
                }
                false
            })
            .count()
            .to_result()
    }

    fn solve_version02(&self, input: ProblemInput, _is_sample: bool) -> ProblemResult {
        /*
        STONE: P + t * Q
        Hailstones: P[i] + t * Q[i]

        For any hailstone i we find an t[i]
        P + t[i] * Q = P[i] + t[i] * Q[i]
        (P - P[i]) + t[i] * (Q - Q[i]) = 0

        If this line goes through the origin (0), then (P - P[i]) and (Q - Q[i]) are linearly dependent, and thus
        (P - P[i]) x (Q - Q[i]) = 0

        For each dimension:
        (P.y - P[i].y) * (Q.z - Q[i].z) - (P.z - P[i].z) * (Q.y - Q[i].y) = 0
        (P.z - P[i].z) * (Q.x - Q[i].x) - (P.x - P[i].x) * (Q.z - Q[i].z) = 0
        (P.x - P[i].x) * (Q.y - Q[i].y) - (P.y - P[i].y) * (Q.x - Q[i].x) = 0

        Expand and sort terms with unknowns to LHS (use dimension with xy as example)
        You arrive at the same equation by solving for the parameter and set them equal for two dimensions
        (P.x * Q.y - P.x * Q[i].y - P[i].x * Q.y) + (- P.y * Q.x + P.y * Q[i].x + P[i].y * Q.x) = P[i].y * Q[i].x - P[i].x * Q[i].y

        Now insert two different values for i (two different hailstones), and invert one equation, as a x b = - (b x a)
          P.x * Q.y - P.x * Q[0].y - P[0].x * Q.y - P.y * Q.x + P.y * Q[0].x + P[0].y * Q.x =   P[0].y * Q[0].x - P[0].x * Q[0].y
        - P.x * Q.y + P.x * Q[1].y + P[1].x * Q.y + P.y * Q.x - P.y * Q[1].x - P[1].y * Q.x = - P[1].y * Q[1].x + P[1].x * Q[1].y

        Add them up to eliminate non linear terms
        - P.x * Q[0].y - P[0].x * Q.y + P.y * Q[0].x + P[0].y * Q.x + P.x * Q[1].y + P[1].x * Q.y - P.y * Q[1].x - P[1].y * Q.x
        = P[0].y * Q[0].x - P[0].x * Q[0].y - P[1].y * Q[1].x + P[1].x * Q[1].y

        Sort and group by unknowns
        [(-Q[0].y + Q[1].y) (Q[0].x - Q[1].x) (0) (P[0].y - P[1].y) (-P[0].x + P[1].x) (0)] * [P.x P.y P.z Q.x Q.y Q.z]^T
        = P[0].y * Q[0].x - P[0].x * Q[0].y - P[1].y * Q[1].x + P[1].x * Q[1].y

        Repeat for all three dimensions, yields 3 equations for 6 unknowns
        [(0) (-Q[0].z + Q[1].z) (Q[0].y - Q[1].y) (0) (P[0].z - P[1].z) (-P[0].y + P[1].y)] * [P.x P.y P.z Q.x Q.y Q.z]^T
        = P[0].z * Q[0].y - P[0].y * Q[0].z - P[1].z * Q[1].y + P[1].y * Q[1].z

        [(Q[0].z - Q[1].z) (0) (-Q[0].x + Q[1].x) (-P[0].z + P[1].z) (0) (P[0].x - P[1].x)] * [P.x P.y P.z Q.x Q.y Q.z]^T
        = P[0].x * Q[0].z - P[0].z * Q[0].x - P[1].x * Q[1].z + P[1].z * Q[1].x

        [(-Q[0].y + Q[1].y) (Q[0].x - Q[1].x) (0) (P[0].y - P[1].y) (-P[0].x + P[1].x) (0)] * [P.x P.y P.z Q.x Q.y Q.z]^T
        = P[0].y * Q[0].x - P[0].x * Q[0].y - P[1].y * Q[1].x + P[1].x * Q[1].y

        Repeat for second different pair of hailstones to have linear equation system to solve, with matrix A and solution b:
        (0)                 (-Q[0].z + Q[1].z)  (Q[0].y - Q[1].y)   (0)                 (P[0].z - P[1].z)   (-P[0].y + P[1].y)
        (Q[0].z - Q[1].z)   (0)                 (-Q[0].x + Q[1].x)  (-P[0].z + P[1].z)  (0)                 (P[0].x - P[1].x)
        (-Q[0].y + Q[1].y)  (Q[0].x - Q[1].x)   (0)                 (P[0].y - P[1].y)   (-P[0].x + P[1].x)  (0)
        (0)                 (-Q[0].z + Q[2].z)  (Q[0].y - Q[2].y)   (0)                 (P[0].z - P[2].z)   (-P[0].y + P[2].y)
        (Q[0].z - Q[2].z)   (0)                 (-Q[0].x + Q[2].x)  (-P[0].z + P[2].z)  (0)                 (P[0].x - P[2].x)
        (-Q[0].y + Q[2].y)  (Q[0].x - Q[2].x)   (0)                 (P[0].y - P[2].y)   (-P[0].x + P[2].x)  (0)

        P[0].z * Q[0].y - P[0].y * Q[0].z - P[1].z * Q[1].y + P[1].y * Q[1].z
        P[0].x * Q[0].z - P[0].z * Q[0].x - P[1].x * Q[1].z + P[1].z * Q[1].x
        P[0].y * Q[0].x - P[0].x * Q[0].y - P[1].y * Q[1].x + P[1].x * Q[1].y
        P[0].z * Q[0].y - P[0].y * Q[0].z - P[2].z * Q[2].y + P[2].y * Q[2].z
        P[0].x * Q[0].z - P[0].z * Q[0].x - P[2].x * Q[2].z + P[2].z * Q[2].x
        P[0].y * Q[0].x - P[0].x * Q[0].y - P[2].y * Q[2].x + P[2].x * Q[2].y
        */

        let hailstones = Self::parse(input);

        // Pick 3 arbitrary hailstones
        let (p0, p1, p2) = (hailstones[0].pos, hailstones[1].pos, hailstones[2].pos);
        let (q0, q1, q2) = (hailstones[0].vel, hailstones[1].vel, hailstones[2].vel);

        let a = vec![
            vec![0, (-q0.z + q1.z), (q0.y - q1.y), 0, (p0.z - p1.z), (-p0.y + p1.y)],
            vec![(q0.z - q1.z), 0, (-q0.x + q1.x), (-p0.z + p1.z), 0, (p0.x - p1.x)],
            vec![(-q0.y + q1.y), (q0.x - q1.x), 0, (p0.y - p1.y), (-p0.x + p1.x), 0],
            vec![0, (-q0.z + q2.z), (q0.y - q2.y), 0, (p0.z - p2.z), (-p0.y + p2.y)],
            vec![(q0.z - q2.z), 0, (-q0.x + q2.x), (-p0.z + p2.z), 0, (p0.x - p2.x)],
            vec![(-q0.y + q2.y), (q0.x - q2.x), 0, (p0.y - p2.y), (-p0.x + p2.x), 0],
        ];
        let b = vec![
            p0.z * q0.y - p0.y * q0.z - p1.z * q1.y + p1.y * q1.z,
            p0.x * q0.z - p0.z * q0.x - p1.x * q1.z + p1.z * q1.x,
            p0.y * q0.x - p0.x * q0.y - p1.y * q1.x + p1.x * q1.y,
            p0.z * q0.y - p0.y * q0.z - p2.z * q2.y + p2.y * q2.z,
            p0.x * q0.z - p0.z * q0.x - p2.x * q2.z + p2.z * q2.x,
            p0.y * q0.x - p0.x * q0.y - p2.y * q2.x + p2.x * q2.y,
        ];

        let solution = algebra::solve_system(Matrix::new(a), b);
        // println!("{solution:?}");
        solution.iter().take(3).sum::<BigInt>().to_result()
    }
}
