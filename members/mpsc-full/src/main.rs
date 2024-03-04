//! what happens when we send a lot of items to an mpsc?

use std::sync::mpsc;

const NUMS: &[u8] = &[1, 3, 3, 7, 0xd, 0xe, 0xa, 0xd, 0xb, 0xe, 0xe, 0xf];

fn main() -> anyhow::Result<()> {
    let (sender, receiver) = mpsc::channel();

    for n in NUMS {
        sender.send(n).unwrap();
    }
    drop(sender);

    let mut collect = Vec::new();

    while let Ok(n) = receiver.recv() {
        collect.push(*n);
    }

    for i in NUMS {
        assert!(collect.contains(i))
    }

    println!("it stores the values, even when the sender is dropped");

    Ok(())
}
