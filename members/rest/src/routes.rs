use std::convert::Infallible;

use libpt::log::info;
use warp::{Filter, Rejection, Reply};

use crate::Store;

pub fn with_store(store: Store) -> impl Filter<Extract = (Store,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}

pub fn routes(store: Store) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_store(store.clone())
}

// GET /api/v1/store
pub fn get_store(store: Store) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "v1" / "store")
        .and(warp::get())
        .and(with_store(store))
        .then(|store: Store| async move {
            info!("GET /api/v1/store");
            warp::reply::json(&store.get().await)
        })
}
