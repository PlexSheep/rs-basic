use std::io::{prelude::*, Sink};

use rayon::prelude::*;

// if we make these larger, our computer can be used as a heater🔥
type Danum = u16;
const CAP: usize = 1 << 14;

#[tokio::main]
async fn main() {
    // Lets say that we want to add many numbers FAST
    let mut range: Vec<Danum> = Vec::with_capacity(CAP);
    // Initialize the values, probably zero
    unsafe {
        range.set_len(range.capacity());
    }
    let now = std::time::Instant::now();
    range.par_iter_mut().for_each(|num| {
        let mut sink = Sink::default();
        while *num < Danum::MAX {
            *num += 1; // cannot use `+=` on type `&u8`
            let _ = write!(sink, "{num}"); // just to disable the compiler from calculating it all
                                           // beforehand
        }
    });
    let sum: u128 = { range.par_iter().map(|n| *n as u128).sum::<u128>() };
    let eq = sum == CAP as u128 * Danum::MAX as u128;
    println!(
        "log cap: {}\nit worked: {eq}\nsum: {sum}\nlog_2(sum): {}\ntook: {:?}\nused threads: {}",
        CAP.ilog2(),
        sum.ilog2(),
        now.elapsed(),
        rayon::current_num_threads()
    );
}
