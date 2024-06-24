use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use rand::seq::SliceRandom;
use std::{collections::HashMap, hash::Hash};

pub const ALPHABET: &str = "qwertzuiopasdfghjklyxcvbnmQWERTZUIOPASDFGHJKLYXCVBNM";
pub const TOK_LEN: usize = 40;
pub type HashedToken = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
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
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn hash(&self, salt: &SaltString) -> HashedToken {
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(self.as_bytes(), salt)
            .expect("could not hash")
            .to_string();
        println!("hashed token: {hash}");
        hash
    }

    fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct User {
    data: String,
    hashed_tok: String,
}

impl User {
    fn new(data: String, salt: &SaltString) -> (Self, Token, HashedToken) {
        let token = Token::new();
        let hashed_tok = token.hash(salt);
        (
            User {
                data,
                hashed_tok: hashed_tok.clone(),
            },
            token,
            hashed_tok,
        )
    }
}

pub struct Store {
    users: HashMap<HashedToken, User>,
    salt: SaltString,
}

impl Store {
    fn new() -> Self {
        let salt = SaltString::generate(&mut OsRng);
        Self {
            salt,
            users: HashMap::new(),
        }
    }
    fn register(&mut self) -> Token {
        let (user, token, hashed_tok) = User::new("garbage data".into(), &self.salt);
        self.users.insert(hashed_tok, user);
        token
    }
    fn login(&self, tok: Token) -> Option<&User> {
        self.users.get(&tok.hash(&self.salt))
    }
}

fn main() {
    // create the user store
    // In a real application, this would be deserialized from a file or lazy-loaded from a database
    let mut store = Store::new();
    for _ in 0..4 {
        // register a few users as noise
        let _ = store.register();
    }
    // create our user and keep the token for that user this time
    let tok = store.register();

    println!("token of our user is: {tok:?}");

    // try to log in with our user
    let logged_in = store.login(tok.clone());

    // did it work?
    assert!(logged_in.is_some());
    dbg!(logged_in.unwrap());
}
