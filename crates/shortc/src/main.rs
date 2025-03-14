use anyhow::Result;
use std::{
    io::{self, prelude::*},
    net,
};

// This is just a very simple tcp server/client that connects to itself

const ADDR: &str = "127.0.0.1:9911";

fn main() -> Result<()> {
    let listen = net::TcpListener::bind(ADDR)?;
    let mut client = net::TcpStream::connect(ADDR)?;
    let _sink = io::sink();
    client.write_all(b"foo")?;
    let mut com = listen.accept()?;
    com.0.write_all(b"bak")?;
    let mut buf = [0; 3];
    let _ = client.read(&mut buf)?;
    println!("{buf:x?}");
    Ok(())
}
