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
        let password = Self::generate_password();
        let notes = "A random example password".to_string();
        Passwords {
            inner: vec![Password::new(1, password, notes)],
        }
    }

    pub fn update_password(&mut self, id: i32, password: String, notes: String) {
        self.inner.iter_mut().for_each(|p| {
            if p.id == id {
                *p = Password::new(id, password.to_string(), notes.to_string())
            }
        })
    }

    pub fn add_password(&mut self, notes: String) {
        let id = match self.inner.iter().map(|p| p.id).max() {
            Some(max_id) => max_id + 1,
            None => 1,
        };
        let password = Self::generate_password();
        let password = Password::new(id, password, notes);
        self.inner.push(password);
    }

    pub fn delete_password(&mut self, id: i32) {
        if let Some(index) = self.inner.iter().position(|x| x.id == id) {
            self.inner.remove(index);
        };
    }

    fn generate_password() -> String {
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
        pg.generate_one().unwrap()
    }
}
