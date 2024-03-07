use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use itertools::Itertools;
use num::{BigInt, Integer, One, Signed, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3<T: Integer> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Integer> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> T
    where
        T: Signed,
    {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<T: Integer> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Integer> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Integer + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Integer + SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Row-major implementation
pub struct Matrix<T: Integer> {
    pub values: Vec<Vec<T>>,
    pub size: usize,
}

impl<T: Integer + Clone + Copy> Matrix<T> {
    pub fn new(values: Vec<Vec<T>>) -> Self {
        assert_eq!(values.len(), values[0].len());
        Self {
            size: values.len(),
            values,
        }
    }

    pub fn det<D: Zero + One + Clone + AddAssign + Neg<Output = D>>(&self) -> D
    where
        D: std::convert::From<T>,
    {
        if self.size == 2 {
            return D::from(self.values[0][0] * self.values[1][1] - self.values[1][0] * self.values[0][1]);
        }

        // Laplace expansion along first row
        let mut val = D::zero();
        let mut sign = D::one();
        for c in 0..self.size {
            let factor = D::from(self.values[0][c]);
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

impl<T: Integer + Copy> Mul<Vec3<T>> for &Matrix<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        assert_eq!(self.size, 3);
        Vec3 {
            x: self.values[0][0] * rhs.x + self.values[0][1] * rhs.y + self.values[0][2] * rhs.z,
            y: self.values[1][0] * rhs.x + self.values[1][1] * rhs.y + self.values[1][2] * rhs.z,
            z: self.values[2][0] * rhs.x + self.values[2][1] * rhs.y + self.values[2][2] * rhs.z,
        }
    }
}

/// Solves A*x = b
pub fn solve_system<T: Copy + Integer + Clone + Zero + Debug>(a: Matrix<T>, b: Vec<T>) -> Vec<T>
where
    BigInt: std::convert::From<T>,
    T: std::convert::TryFrom<BigInt>,
    <T as std::convert::TryFrom<BigInt>>::Error: Debug,
{
    let det_a: BigInt = a.det();
    let mut x = vec![T::zero(); b.len()];
    for i in 0..b.len() {
        let temp_matrix = Matrix::new(
            a.values
                .iter()
                .enumerate()
                .map(|(ri, r)| [&r[0..i], &[b[ri]], &r[i + 1..]].concat())
                .collect_vec(),
        );

        x[i] = (temp_matrix.det::<BigInt>() / det_a.clone()).try_into().unwrap();
    }

    x
}
