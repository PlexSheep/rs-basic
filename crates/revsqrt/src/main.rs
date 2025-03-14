use std::process::exit;

use revsqrt::*;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        usage(&args[0]);
        exit(1);
    }
    let n: f32 = match args[1].parse() {
        Ok(n) => n,
        Err(err) => {
            eprintln!("could not parse: {err}\n");
            usage(&args[0]);
            exit(1);
        }
    };

    let mut start = std::time::Instant::now();
    let rr = regular_inverse_sqrt(n);
    let rdur = start.elapsed();

    start = std::time::Instant::now();
    let fr = fast_inverse_sqrt(n);
    let fdur = start.elapsed();

    println!(
        "regular\tinverse square root of {n}: {rr}\t(took {rdur:?})\n\
    fast   \tinverse square root of {n}: {fr}\t(took {fdur:?})\n\n\
    The timings are not accurate. Benchmarks show ~170 ps for each.\n\
    The CPU has a instruction for sqrt, and 1/x is simple division, so\n\
    an algorithm like fast inverse square root can hardly be faster."
    );
}

fn usage(invocation: &str) {
    println!("usage:\n\n{invocation} NUMBER # (i.e. 1.3)");
}
