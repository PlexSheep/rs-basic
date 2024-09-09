use libpt::log::{debug, info};

mod routes;
use routes::*;
mod store;
use store::*;

#[tokio::main]
async fn main() {
    libpt::log::Logger::build_mini(Some(libpt::log::Level::DEBUG)).expect("could not init logger");
    let store = Store::new();
    debug!("spawning data_processing task: {store:#?}");
    tokio::spawn(data_processing(store.clone()));
    info!("starting webserver");
    warp::serve(get_store(store))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
