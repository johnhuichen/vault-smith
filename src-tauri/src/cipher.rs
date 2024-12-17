use std::fmt::Debug;
use std::io::{Read, Write};

use cocoon::Cocoon;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum CipherError {
    #[snafu(display("Encryption/Decryption Error: {:?}", source))]
    Cocoon { source: cocoon::Error },
}

#[derive(Debug)]
pub struct Cipher {
    masterkey: String,
}

impl Cipher {
    pub fn new(masterkey: &str) -> Self {
        Cipher {
            masterkey: masterkey.to_string(),
        }
    }

    pub fn dump(&self, data: Vec<u8>, writer: &mut impl Write) -> Result<(), CipherError> {
        let mut cocoon = Cocoon::new(self.masterkey.as_bytes());
        cocoon.dump(data, writer).context(CocoonSnafu)?;
        Ok(())
    }

    pub fn parse(&self, reader: &mut impl Read) -> Result<Vec<u8>, CipherError> {
        let cocoon = Cocoon::new(self.masterkey.as_bytes());
        cocoon.parse(reader).context(CocoonSnafu)
    }
}
