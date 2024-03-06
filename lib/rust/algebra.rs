use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use itertools::Itertools;
use num::{BigInt, One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl Vec3 {
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> i128 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Row-major implementation
pub struct Matrix {
    pub values: Vec<Vec<i128>>,
    pub size: usize,
}

impl Matrix {
    pub fn new(values: Vec<Vec<i128>>) -> Self {
        assert_eq!(values.len(), values[0].len());
        Self {
            size: values.len(),
            values,
        }
    }

    pub fn det(&self) -> BigInt {
        if self.size == 2 {
            return BigInt::from(self.values[0][0] * self.values[1][1] - self.values[1][0] * self.values[0][1]);
        }

        // Laplace expansion along first row
        let mut val = BigInt::zero();
        let mut sign = BigInt::one();
        for c in 0..self.size {
            let factor = self.values[0][c];
            let submat = Matrix::new(
                self.values
                    .iter()
                    .skip(1)
                    .map(|r| [&r[0..c], &r[c + 1..self.size]].concat())
                    .collect_vec(),
            );
            val += sign.clone() * factor * submat.det();
            sign = -sign;
        }

        val
    }
}

impl Mul<Vec3> for &Matrix {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        assert_eq!(self.size, 3);
        Vec3 {
            x: self.values[0][0] * rhs.x + self.values[0][1] * rhs.y + self.values[0][2] * rhs.z,
            y: self.values[1][0] * rhs.x + self.values[1][1] * rhs.y + self.values[1][2] * rhs.z,
            z: self.values[2][0] * rhs.x + self.values[2][1] * rhs.y + self.values[2][2] * rhs.z,
        }
    }
}

/// Solves A*x = b
pub fn solve_system(a: Matrix, b: Vec<i128>) -> Vec<BigInt> {
    let det_a = a.det();
    let mut x = vec![BigInt::zero(); b.len()];
    for i in 0..b.len() {
        let temp_matrix = Matrix::new(
            a.values
                .iter()
                .enumerate()
                .map(|(ri, r)| [&r[0..i], &[b[ri]], &r[i + 1..]].concat())
                .collect_vec(),
        );

        x[i] = temp_matrix.det() / det_a.clone();
    }

    x
}
