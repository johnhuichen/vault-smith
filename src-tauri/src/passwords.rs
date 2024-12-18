use borsh::{BorshDeserialize, BorshSerialize};
use passwords::PasswordGenerator;
use rand::Rng;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Password {
    id: i32,
    password: String,
    notes: String,
}

impl Password {
    pub fn new(id: i32, password: String, notes: String) -> Self {
        Self {
            id,
            password,
            notes,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum PasswordsError {}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Passwords {
    pub inner: Vec<Password>,
}

impl Passwords {
    pub fn random_one() -> Self {
        let length = rand::thread_rng().gen_range(12..20);
        let pg = PasswordGenerator {
            length,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        };
        let password = pg.generate_one().unwrap();
        let notes = "A random password".to_string();
        Passwords {
            inner: vec![Password::new(1, password, notes)],
        }
    }

    // let id = uuid::Uuid::new_v4().to_string();
    //
    //
    // pub fn contains_key(&self, domain: &str) -> bool {
    //     self.inner.contains_key(domain)
    // }
    //
    // pub fn insert(&mut self, domain: String, password: String) -> Option<String> {
    //     self.inner.insert(domain, password)
    // }
    //
    // pub fn update(&mut self, domain: String, password: String) {
    //     *self.inner.entry(domain).or_insert(password) = password.to_string()
    // }
    //
    // pub fn delete(&mut self, domain: &str) -> Option<String> {
    //     self.inner.remove(domain)
    // }
}
