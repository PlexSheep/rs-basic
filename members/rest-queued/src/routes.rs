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

mod test {
    use warp::{
        http::StatusCode,
        hyper::{self, body::*},
        reply::{Json, Reply},
    };

    use std::convert::From;
    use std::str::FromStr;
    use hyper::body::Bytes;

    #[tokio::test]
    async fn test_register_and_get() {
        let store = crate::Store::new();
        let filter = super::get_register(store.clone());

        let response: warp::reply::Response = warp::test::request()
            .path("/api/v1/register")
            .filter(&filter)
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let body_raw = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_raw).unwrap();

        assert!(body_json.is_object());
        let id: crate::Id = crate::Id::from_str(
            &body_json
                .get("id")
                .expect("response has no field 'id'")
                .to_owned()
                .to_string(),
        )
        .unwrap();
        let token: crate::Token = crate::Token::from_str(
            &body_json
                .get("token")
                .expect("response has no field 'token'")
                .to_owned()
                .to_string(),
        )
        .unwrap();

        assert_eq!(id.len(), crate::ID_LEN);
        assert_eq!(token.len(), crate::TOK_LEN);

        let filter = super::get_items(store);
        let response: warp::reply::Response = warp::test::request()
            .path(format!("/api/v1/items/?id={id}").as_str())
            .header("Token", token.to_string())
            .filter(&filter)
            .await
            .unwrap()
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let body_raw = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_raw).unwrap();
        assert!(body_json.is_array());

        for i in 0..2 {
            assert!(body_json[i].is_object());
            let item: crate::Item = serde_json::from_value(body_json[i].clone().take()).unwrap();
            assert_eq!(item.seq, i);
        }
    }
}
