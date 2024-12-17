use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Password {
    password: String,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Passwords {
    inner: Vec<Password>,
}

impl Passwords {
    pub fn empty() -> Self {
        Passwords { inner: Vec::new() }
    }

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
