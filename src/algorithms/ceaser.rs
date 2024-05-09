use super::{Crypto, Utils, ALPHABET, ALPHABET_SIZE};
use crate::algorithms::CryptoError;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Ceaser {
    data: String,
    key: i32,
}

impl Crypto for Ceaser {
    fn encrypt(&self) -> Result<String, super::CryptoError> {
        if self.key < 1 {
            return Err(CryptoError::InvalidKey);
        }

        let mut encripted: String = String::new();

        for c in self.data.to_lowercase().chars() {
            let index: i32 = Utils::unicode_to_int(c);

            if index < 0 || index > 26 {
                return Err(CryptoError::InvalidData);
            }

            let shifted_index = Utils::modulo(index + self.key, ALPHABET_SIZE);

            encripted.push(ALPHABET[shifted_index as usize])
        }

        Ok(encripted)
    }

    fn decrypt(&self) -> Result<String, CryptoError> {
        if self.key < 1 {
            return Err(CryptoError::InvalidKey);
        }

        let mut decripted: String = String::new();

        for c in self.data.to_lowercase().chars() {
            let index: i32 = Utils::unicode_to_int(c);

            if index < 0 || index > 26 {
                return Err(CryptoError::InvalidData);
            }

            let shifted_index = Utils::modulo(index - self.key, ALPHABET_SIZE);

            decripted.push(ALPHABET[shifted_index as usize]);
        }

        Ok(decripted)
    }
}
