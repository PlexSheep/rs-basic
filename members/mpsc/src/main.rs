//! Example on how to use std::mpsc with 2 threads

use anyhow::Result;
use std::io::Write;
use std::str::FromStr;
use std::sync::{Arc, Barrier};
use std::{sync::mpsc, thread};

#[derive(Clone, Debug)]
struct Message {
    payload: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{{}}}", self.payload)
    }
}

impl std::str::FromStr for Message {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self {
            payload: s.to_string(),
        })
    }
}

fn printer(receiver: mpsc::Receiver<Message>, barrier: Arc<Barrier>) -> Result<()> {
    let mut stdout = std::io::stdout();
    loop {
        let msg = receiver.recv()?;
        println!("{msg}");
        stdout.flush()?;
        barrier.wait(); // wait until the main thread wants us to print
    }
}

fn main() -> Result<()> {
    let (sender, receiver) = mpsc::channel();
    let barrier = Arc::new(Barrier::new(2));
    let barrier_printer = barrier.clone();
    thread::spawn(|| printer(receiver, barrier_printer).expect("printer error"));
    let mut msg;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        buf.clear();
        print!("> ");
        stdout.flush()?;
        let _ = stdin.read_line(&mut buf)?;
        buf = buf.replace('\n', "");
        if buf.is_empty() {
            continue;
        }
        msg = Message::from_str(&buf).unwrap();
        sender.send(msg)?;
        barrier.wait(); // wait until the printer is done printing the message, so we dont mix
                        // stdout prints (we use print instead of println)
    }
}
