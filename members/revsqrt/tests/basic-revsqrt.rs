use std::iter::zip;





// is n about the same as m?
// This is actually not so easy! How do you measure "about same"ness?
// Also, it is not transitive, as 1 ≈ 1.1 ≈ 1.2 ≈ 1.3 ≈ ... ≈ 2 ≈ ... ≈ 3 ≈ ... ≈ infinity, that's
// a thought of me at least?
#[inline]
fn about_same(n: f32, m: f32) -> bool {
    // dbg!((n, m));
    // dbg!((n - m).abs());
    // dbg!(calc_gate(n, m));
    // dbg!((n - m).abs() < calc_gate(n, m));
    (n - m).abs() <= calc_gate(n, m)
}

#[inline]
fn calc_gate(n: f32, m: f32) -> f32 {
    0.01 + ((n.abs().sqrt().min(m.abs().sqrt())).abs() / 10f32)
}

#[test]
fn test_calc_fast_rsqrt() {
    assert_ne!(0.0, revsqrt::fast_inverse_sqrt(rand::random()))
}

#[test]
fn test_calc_regular_rsqrt() {
    assert_ne!(0.0, revsqrt::regular_inverse_sqrt(rand::random()))
}

#[test]
fn test_calc_specific_fast_rsqrt() {
    let params: &[f32] = &[1.0, 1.1, 100.0, 1337.0, 123.456_79, 1337.1337];
    let results: &[f32] = &[
        1.0,
        0.953_462_6,
        0.1,
        0.027_348_55,
        0.09,
        0.027_347_183,
    ];
    for (n, m) in zip(params, results) {
        assert!(about_same(revsqrt::fast_inverse_sqrt(*n), *m))
    }
}

#[test]
fn test_calc_specific_reqular_rsqrt() {
    let params: &[f32] = &[1.0, 1.1, 100.0, 1337.0, 123.456_79, 1337.1337];
    let results: &[f32] = &[
        1.0,
        0.953_462_6,
        0.1,
        0.027_348_55,
        0.09,
        0.027_347_183,
    ];
    for (n, m) in zip(params, results) {
        assert_eq!(revsqrt::regular_inverse_sqrt(*n), *m)
    }
}

#[allow(clippy::assertions_on_constants)]
#[test]
#[ignore] // this test confuses the CI
fn test_fail() {
    println!("the stdout will be printed on fail!");
    assert!(false)
}
