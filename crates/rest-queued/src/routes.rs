use std::{collections::HashMap, convert::Infallible, str::FromStr};

use libpt::log::{debug, error, info};
use warp::{
    filters::BoxedFilter,
    http::StatusCode,
    reject::{MissingHeader, Rejection},
    reply::{self, Json},
    Filter, Reply,
};

use crate::{Client, Id, Store, StoreErr, Token};

pub fn with_store(store: Store) -> impl Filter<Extract = (Store,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}

pub fn routes(store: Store) -> BoxedFilter<(impl Reply,)> {
    get_items(store.clone())
        .or(get_register(store.clone()))
        .recover(handle_rejection)
        .boxed()
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        debug!("page not found");
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<MissingHeader>() {
        debug!("{e}");
        Ok(reply::with_status(
            "MISSING_HEADER: TOKEN",
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(e) = err.find::<StoreErr>() {
        debug!("{e}");
        Ok(reply::with_status("UNAUTHENTICATED", StatusCode::FORBIDDEN))
    } else {
        error!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn item_getter(
    param: HashMap<String, String>,
    token: Token,
    store: Store,
) -> Result<warp::reply::Json, warp::Rejection> {
    info!("GET /api/v1/items");
    if let Some(id) = param.get("id") {
        let id = Id::from_str(id).unwrap();
        let client = match store.login(id, token).await {
            Ok(client) => client,
            Err(unauth) => return Err(warp::reject::custom(unauth)),
        };
        Ok(warp::reply::json(&client.get_items(store.clone()).await))
    } else {
        Err(warp::reject())
    }
}

// GET /api/v1/items
pub fn get_items(store: Store) -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "items")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::<Token>("Token"))
        .and(with_store(store))
        // .and(warp::body::content_length_limit(2 << 13))
        .and_then(item_getter)
        .boxed()
}

// GET /api/v1/register
pub fn get_register(
    store: Store,
) -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / "register")
        .and(warp::get())
        .and(with_store(store))
        // .and(warp::body::content_length_limit(2 << 13))
        .then(|store: Store| async move {
            info!("GET /api/v1/register");
            let client = Client::new();
            let response = warp::reply::json(&client);
            store.register_client(client).await;
            response
        })
        .boxed()
}
