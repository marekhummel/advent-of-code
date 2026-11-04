use std::ops::{AddAssign, Rem};

use itertools::Itertools;
use num::{Integer, Signed};

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / a.gcd(&b)
}

pub fn bits<const L: usize>(mut n: u128) -> [bool; L] {
    let mut bits = [false; L];
    for i in (0..L).rev() {
        bits[i] = n & 1 == 1;
        n >>= 1;

        if n < 1 {
            break;
        }
    }

    bits
}

/// Unused, but saved for later (originated from attempts in 2023-24)
pub fn gauss_jordan(a: &mut [&mut [f64]], b: &mut [f64]) {
    // Find pivot and create 1
    let p = a[0][0];
    a.get_mut(0).unwrap().iter_mut().for_each(|v| *v /= p);
    *b.get_mut(0).unwrap() /= p;

    // Return if only length 1
    if a.len() == 1 {
        return;
    }

    // Adjust b
    let pivot_b = b[0];
    b.iter_mut()
        .enumerate()
        .skip(1)
        .for_each(|(i, bi)| *bi -= a[i][0] * pivot_b);

    // Adjust rows to create 0s in same column underneath the 1
    let pivot_row = a[0].to_vec();
    a.iter_mut().skip(1).for_each(|row| {
        let f = row[0];
        row.iter_mut().enumerate().for_each(|(i, v)| *v -= f * pivot_row[i]);
    });

    // Continue with sub matrices until upper triangle is given
    gauss_jordan(&mut a.iter_mut().map(|r| &mut r[1..]).collect_vec()[1..], &mut b[1..]);

    // Adjust rows to create 0s above 1s on diagonal
    let delta_b = a[0][1..].iter().zip(b[1..].iter()).map(|(ai, bi)| ai * bi).sum::<f64>();
    *b.get_mut(0).unwrap() -= delta_b;

    let subs = a
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, r)| r.iter().map(|v| v * a[0][i]).collect_vec())
        .collect_vec();
    a.get_mut(0)
        .unwrap()
        .iter_mut()
        .skip(1)
        .enumerate()
        .for_each(|(i, v)| *v -= subs[i].iter().sum::<f64>());
}

/// Simple primality test, could be improved with miller rabin if needed
pub fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }

    if n & 1 == 0 {
        return n == 2;
    }

    let root = (n as f64).sqrt() as u128;
    (3..=root).step_by(2).all(|f| n % f != 0)
}

pub fn mod_inverse<T: Copy + Integer + Signed>(n: T, modulus: T) -> Option<T> {
    match extended_gcd(n, modulus) {
        (g, x, _) if g.is_one() => Some((x % modulus + modulus) % modulus),
        _ => None,
    }
}

fn extended_gcd<T: Copy + Integer + Signed>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn chinese_remainder<T>(residues: &[T], modulii: &[T]) -> Option<T>
where
    T: Copy + Integer + Signed + std::iter::Product<T> + Rem<Output = T> + AddAssign,
{
    let prod = modulii.iter().copied().product::<T>();

    let mut sum = T::zero();
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inverse(p, modulus)? * p
    }

    Some(sum % prod)
}
