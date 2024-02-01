use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use tokio::time::{interval, Instant};

// if we make these larger, our computer can be used as a heater🔥
type Danum = u16;
const CAP: usize = 1 << 16;
const M: u128 = CAP as u128 * Danum::MAX as u128;

fn status(start: &Instant, range: &Vec<Danum>) -> bool {
    let sum: u128 = { range.par_iter().map(|n| *n as u128).sum::<u128>() };
    if sum < 1 {
        return false;
    }
    let progress = sum as f64 / M as f64;
    let eq = sum == M;
    println!(
        r#"
    done:               {}
    current threads:    {}
    progress:           {}%
    log_2(capacity):    {}
    log_2(sum):         {}
    cap:                {}
    sum:                {}
    M:                  {}
    took:               {:?}
    "#,
        eq,
        rayon::current_num_threads(),
        progress * 100.0,
        CAP.ilog2(),
        sum.ilog2(),
        CAP,
        sum,
        M,
        start.elapsed(),
    );
    eq
}

#[tokio::main]
async fn main() {
    // Lets say that we want to add many numbers FAST
    let mut range: Vec<Danum> = Vec::with_capacity(CAP);
    // Initialize the values, probably zero
    unsafe {
        range.set_len(range.capacity());
    }

    let start = Instant::now();
    let lock = Arc::new(Mutex::new(range));
    let lock2 = lock.clone();
    rayon::spawn(move || {
        const FOOF: usize = 16;
        for i in 0..FOOF {
            let mut range = lock.lock().unwrap();
            range.par_iter_mut().skip(i).step_by(FOOF).for_each(|num| {
                for _ in 0..Danum::MAX {
                    *num += 1;
                    let _ = write!(std::io::Sink::default(), "{num}");
                }
            });
        }
    });
    let mut ticker = interval(tokio::time::Duration::from_millis(500));
    loop {
        ticker.tick().await;
        if status(&start, &lock2.lock().unwrap()) {
            break;
        };
    }
}
