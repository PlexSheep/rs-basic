use rayon::prelude::*;
use std::io::prelude::*;
use tokio::time::Instant;

// if we make these larger, our computer can be used as a heater🔥
type Danum = u16;
const EXP: usize = 12; // FIXME: If this goes lower than 7, somehow the mpsc breaks?
const CAP: usize = 1 << EXP;
const M: u128 = CAP as u128 * Danum::MAX as u128;

fn status(start: &Instant, info: (u128, usize), separate: usize) -> bool {
    if info.0 < 1 {
        return false;
    }
    let progress = info.0 as f64 / M as f64;
    assert!(info.0 <= M);
    let eq = info.0 == M;
    println!(
        r#"
    done:               {}
    current threads:    {}
    progress:           {}%
    log_2(capacity):    {}
    log_2(sum):         {}
    cap:                {}
    sum:                {}
    sep:                {}/{}
    M:                  {}
    took:               {:?}
    "#,
        eq,
        rayon::current_num_threads(),
        progress * 100.0,
        CAP.ilog2(),
        info.0.ilog2(),
        CAP,
        info.0,
        info.1,
        separate,
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
    let separate: usize = 1 << (EXP / 2);
    // stops earlier sometimes
    let (sender, recv) = std::sync::mpsc::channel();
    rayon::spawn(move || {
        for i in 0..separate + 1 {
            match sender.send((range.par_iter().map(|n| *n as u128).sum(), i)) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{err}");
                    break;
                }
            }
            range
                .par_iter_mut()
                .skip(i)
                .step_by(separate)
                .for_each(|num| {
                    for _ in 0..Danum::MAX {
                        *num += 1;
                        let _ = write!(std::io::Sink::default(), "{num}");
                    }
                });
        }
        println!("DONE!");
    });
    loop {
        match recv.recv() {
            Ok(tup) => {
                if status(&start, tup, separate) {
                    break;
                }
            }
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
    drop(recv);
}
