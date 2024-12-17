use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Passwords {
    inner: HashMap<String, String>,
}

impl Passwords {
    pub fn empty() -> Self {
        Passwords {
            inner: HashMap::new(),
        }
    }

    pub fn contains_key(&self, domain: &str) -> bool {
        self.inner.contains_key(domain)
    }

    pub fn insert(&mut self, domain: String, password: String) -> Option<String> {
        self.inner.insert(domain, password)
    }

    pub fn update(&mut self, domain: String, password: String) {
        *self.inner.entry(domain).or_insert(password) = password.to_string()
    }

    pub fn delete(&mut self, domain: &str) -> Option<String> {
        self.inner.remove(domain)
    }
}
