use libpt::log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    sync::{atomic::AtomicUsize, Arc},
};
use tokio::sync::Mutex;

use crate::{Client, Id, Token};

pub const TOO_MANY_ITEMS: usize = 2048;

pub static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
pub static LAST_SEQUENCE: AtomicUsize = AtomicUsize::new(0);
pub type Sequence = usize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum StoreErr {
    Unauthenticated(Id),
    NotRegistered(Id),
}
impl warp::reject::Reject for StoreErr {}
impl Display for StoreErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthenticated(id) => {
                write!(f, "unauthenticated get_items request for: {id}",)
            }
            Self::NotRegistered(id) => {
                write!(f, "request with unregistered id: {id}",)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    all: Arc<Mutex<VecDeque<Item>>>,
    clients: Arc<Mutex<HashMap<Id, Client>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub body: serde_json::Value,
    pub seq: Sequence,
}

impl Item {
    fn new(msg: serde_json::Value) -> Self {
        let seq = SEQUENCE.load(std::sync::atomic::Ordering::Relaxed);
        SEQUENCE.store(seq + 1, std::sync::atomic::Ordering::Relaxed);

        Self {
            body: msg,
            seq,
        }
    }
}

#[cfg(debug_assertions)]
impl Drop for Item {
    fn drop(&mut self) {
        debug!("dropping {:?}", self.seq)
    }
}

impl From<serde_json::Value> for Item {
    fn from(value: serde_json::Value) -> Self {
        Item::new(value)
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            all: Arc::new(Mutex::new(
                vec![
                    Item::new(serde_json::json!({"foo": "bar", "value": 0})),
                    Item::new(serde_json::json!({"foo": "bar", "value": 1})),
                    Item::new(serde_json::json!({"foo": "bar", "value": 2})),
                ]
                .into(),
            )),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_item(&self, item: Item) {
        let mut store = self.all.lock().await;
        if store.len() > TOO_MANY_ITEMS {
            warn!(
                "Too many items ({}), removing old ones until okay",
                store.len()
            );
            while let Some(_item) = store.front() {
                if store.len() < TOO_MANY_ITEMS {
                    break;
                }
                store.pop_front();
            }
        }
        store.push_back(item);
    }

    pub async fn status(&self) {
        let clients_len = self.clients.lock().await.len();
        let item_len = self.all.lock().await.len();

        info!("status: {clients_len} clients; {item_len} items");
    }

    pub async fn get_items(&self) -> Vec<Item> {
        let store = self.all.lock().await;
        store.clone().into_iter().collect()
    }

    pub async fn register_client(&self, client: Client) {
        let mut store = self.clients.lock().await;
        store.insert(client.id().clone(), client);
    }

    pub async fn garbage_collect(&self) {
        let mut store = self.all.lock().await;
        let seq = SEQUENCE.load(std::sync::atomic::Ordering::Relaxed);
        let lseq = LAST_SEQUENCE.load(std::sync::atomic::Ordering::Relaxed);
        if seq <= lseq {
            return;
        }
        while let Some(item) = store.front() {
            if item.seq > lseq {
                break;
            }
            store.pop_front().unwrap();
        }
        drop(store); // free the lock
        self.status().await;
    }

    pub(crate) async fn adjust_lseq(&self, newer: Sequence) -> Sequence {
        let lseq = LAST_SEQUENCE.load(std::sync::atomic::Ordering::Relaxed);

        if newer > lseq {
            LAST_SEQUENCE.store(newer, std::sync::atomic::Ordering::Relaxed);
            self.garbage_collect().await;
            newer
        } else {
            lseq
        }
    }

    pub async fn login(&self, id: Id, token: Token) -> Result<Client, StoreErr> {
        let clients = self.clients.lock().await;
        let potential_client = match clients.get(&id) {
            Some(c) => c,
            None => return Err(StoreErr::NotRegistered(id)),
        };

        if potential_client.validate_token(token) {
            // HACK: cloning here is bad
            Ok(potential_client.clone())
        } else {
            Err(StoreErr::Unauthenticated(id))
        }
    }
}

pub async fn data_processing(store: Store) {
    let mut iter = 3;
    loop {
        let item = serde_json::json!({"foo": "bar", "value": iter}).into();
        store.add_item(item).await;

        if iter % 5 == 0 {
            store.status().await;
        }

        iter += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
