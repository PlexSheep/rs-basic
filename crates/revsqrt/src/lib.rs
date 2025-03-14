/// Calculate the inverse square of a number n using the rust std library.
/// This is faster than the fast inverse square root algorithm, because sqrt
/// is a cpu instruction on modern cpus (`fsqrt`). Inverting as 1/x seems to
/// be pretty fast too, just using a regular division.
#[inline]
pub fn regular_inverse_sqrt(n: f32) -> f32 {
    n.sqrt().recip()
}

/// Helper union that lets us convert between [u32] and [f32] for the fast inverse square root
/// algorithm.
union MixedIntFloat {
    f: f32,
    i: u32,
}

/// Interestingly, the benchmark shows that this function is not faster than regular inverse sqrt.
/// This is probably due to the cpu being able to calculate the reverse square root with a regular
/// calculation in two instructions: sqrt and division.
///
/// see https://en.wikipedia.org/wiki/Fast_inverse_square_root
///
/// This is unsafe, but I've decided not to put it in rs-unsafe instead of rs-basic,
/// as I only use this example for benchmarking with criterion.
#[inline]
pub fn fast_inverse_sqrt(n: f32) -> f32 {
    let mut conv: MixedIntFloat = MixedIntFloat { f: n };
    unsafe {
        // reading from a union is unsafe in Rust.
        conv.i = 0x5f3759df - (conv.i >> 1);
        conv.f * (1.5 - n * 0.5 * conv.f * conv.f)
    }
}
