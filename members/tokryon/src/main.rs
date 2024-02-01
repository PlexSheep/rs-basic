use std::sync::atomic::{AtomicU32, Ordering};

use rayon::prelude::*;
use tokio::{
    self,
    time::{interval, Interval},
};

static THE_NUMBER: AtomicU32 = AtomicU32::new(0);

#[inline]
fn incr() {
    // just add it
    THE_NUMBER.store(THE_NUMBER.load(Ordering::Relaxed) + 1, Ordering::Relaxed)
}

#[inline]
fn get() -> u32 {
    THE_NUMBER.load(Ordering::Relaxed)
}
#[inline]
async fn more() {
    rayon::spawn(|| incr())
}

#[tokio::main]
async fn main() {
    // Lets say that we want to add numbers FAST
    println!("The number: {THE_NUMBER:?}");
    incr();
    assert_eq!(get(), 1);
    println!("The number: {THE_NUMBER:?}");
    println!("starting the threads");
    let mut interval = interval(tokio::time::Duration::from_millis(100));
    loop {
        match future::select(do_work(), interval.tick()).await {
            Either::Left((result, _)) => {
                // Our worker completed successfully!
                result?;

                // Do any additional processing here after successful completion
             },

            Either::Right(_) => {
               // The interval has fired - we don't have to wait for the worker anymore
               println!("Interval triggered!");
           }
       };
        tokio::select! {
        _ = interval.tick() => {
        println!("The number: {THE_NUMBER:?}");
            }
        _ = more() => ()
        };
    }
}
