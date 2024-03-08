//! some task produces data [Items](store::Item), and we want clients to be able to get those.
//!
//! We don't want to keep storing all items, so we keep track of what the lowest item is that a
//! client hasnt received yet. The architecture requires clients to register, so while we're at it
//! we hand them a token too, which they will authenticate to us.
//!
//! This way, we have a distribution api, that clients can use to get any messages they have not
//! yet received.

use libpt::log::{debug, info};

mod routes;
use routes::*;
mod store;
use store::*;
mod client;
use client::*;

#[tokio::main]
async fn main() {
    libpt::log::Logger::build_mini(Some(libpt::log::Level::DEBUG)).expect("could not init logger");
    let store = Store::new();
    tokio::spawn(data_processing(store.clone()));
    info!("starting webserver");
    warp::serve(routes(store)).run(([127, 0, 0, 1], 3030)).await;
}
