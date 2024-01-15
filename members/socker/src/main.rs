use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
};

mod pool;
use pool::ThreadPool;

fn main() {
    println!("Starting the server");
    let target_address = "127.0.0.1:7878";
    let pool = ThreadPool::build(4).unwrap();
    let listener = match TcpListener::bind(target_address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Could not start server: {err}");
            exit(1);
        }
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_rd = BufReader::new(&mut stream);
    let request: Vec<_> = buf_rd
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", request);
    match request[0].as_str() {
        "ping" => {
            writeln!(stream, "> pong!").unwrap();
        }
        _ => {
            writeln!(stream, "> command not recognized.").unwrap();
        }
    }
}
