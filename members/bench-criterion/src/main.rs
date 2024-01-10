fn main() {
    let n: f32 = std::env::args().collect::<Vec<String>>()[1]
        .parse::<f32>()
        .expect("bad input number");
    println!("rsqrt:\t\t{}", inverse_sqrt(n));
    println!("f_rsqrt:\t{}", fast_inverse_sqrt(n));
}

fn inverse_sqrt(n: f32) -> f32 {
    1f32 / n.sqrt()
}

union MixedIntFloat {
    f: f32,
    i: u32,
}

/// see https://en.wikipedia.org/wiki/Fast_inverse_square_root
fn fast_inverse_sqrt(n: f32) -> f32 {
    let mut conv: MixedIntFloat = MixedIntFloat { f: n };
    unsafe {
        conv.i  = 0x5f3759df - (conv.i >> 1);
        conv.f *= 1.5f32 - (n* 0.5f32 * conv.f * conv.f);
        conv.f
    }
}
