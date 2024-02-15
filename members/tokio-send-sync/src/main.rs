use std::{ sync::Arc};

use tokio::{net::TcpListener, sync::Mutex};

type TestType = u64;

async fn foo(t: TestType) -> Result<TcpListener, std::io::Error> {
    println!("{t}");
    TcpListener::bind("127.0.0.1:50023").await
}

#[tokio::main()]
async fn main() {
    let testdata: TestType = 1337;
    let arcmut = Arc::new(Mutex::new(testdata));
    tokio::spawn(async move {
        let a = foo(testdata).await;
        a.unwrap().accept().await
    });
}
