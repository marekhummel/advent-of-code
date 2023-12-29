// Unused, but saved for later (originated from attempts in 2023-24)
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
    Self::gauss_jordan(&mut a.iter_mut().map(|r| &mut r[1..]).collect_vec()[1..], &mut b[1..]);

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
