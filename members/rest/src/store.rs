use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Store {
    inner: Arc<Mutex<VecDeque<Item>>>,
}

pub type Item = serde_json::Value;

impl Store {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                vec![
                    serde_json::json!({"foo": "bar", "value": 0}),
                    serde_json::json!({"foo": "bar", "value": 1}),
                    serde_json::json!({"foo": "bar", "value": 2}),
                ]
                .into(),
            )),
        }
    }
    pub async fn add(&self, item: Item) {
        let mut store = self.inner.lock().await;
        if store.len() > 10 {
            store.pop_front().unwrap();
        }
        store.push_back(item);
    }

    pub async fn get(&self) -> Vec<Item> {
        let store = self.inner.lock().await;
        store.clone().into_iter().collect()
    }
}

pub async fn data_processing(store: Store) {
    let mut iter = 3;
    loop {
        let msg = serde_json::json!({"foo": "bar", "value": iter});
        store.add(msg).await;
        iter += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
