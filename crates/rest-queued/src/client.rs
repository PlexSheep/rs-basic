use std::{convert::Infallible, fmt::Display, str::FromStr};

use crate::{store::Sequence, Item, Store};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

pub const ALPHABET: &str = "qwertzuiopasdfghjklyxcvbnmQWERTZUIOPASDFGHJKLYXCVBNM";
pub const TOK_LEN: usize = 40;
pub const ID_LEN: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Client {
    #[serde(flatten)]
    id: Id,
    #[serde(flatten)]
    token: Token,
    last: Sequence,
}

impl Client {
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            token: Token::new(),
            last: 0,
        }
    }

    #[cfg(debug_assertions)]
    #[allow(unused)]
    pub(crate) fn new_debug() -> Self {
        Self {
            id: Id::from_str("myid").unwrap(),
            token: Token::from_str("mytok").unwrap(),
            last: 0,
        }
    }

    pub async fn get_items(&self, store: Store) -> Vec<Item> {
        let items = store.get_items().await;
        if let Some(item) = items.last() {
            store.adjust_lseq(item.seq).await;
        }
        items
    }

    pub fn validate_token(&self, token: Token) -> bool {
        token == self.token
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Id {
    #[serde(rename = "id")]
    inner: String,
}

impl Id {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut data = ALPHABET.to_string().into_bytes();
        data.repeat(ID_LEN);
        data.shuffle(&mut rng);
        Self {
            inner: String::from_utf8(data[..ID_LEN].into()).unwrap(),
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl FromStr for Id {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('"', "");
        Ok(Self { inner: s })
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "token")]
    inner: String,
}

impl Token {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut data = ALPHABET.to_string().into_bytes();
        data.repeat(TOK_LEN);
        data.shuffle(&mut rng);
        Self {
            inner: String::from_utf8(data[..TOK_LEN].to_vec()).unwrap(),
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for Token {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('"', "");
        Ok(Self { inner: s })
    }
}

mod test {
    #[test]
    fn test_new_dbg() {
        let c = super::Client::new_debug();
        assert_eq!(c.token.to_string(), "mytok");
        assert_eq!(c.id.to_string(), "myid");
    }
}
