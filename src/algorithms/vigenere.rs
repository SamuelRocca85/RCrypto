use super::{Crypto, CryptoError, Utils, ALPHABET, ALPHABET_SIZE};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Vigenere {
    data: String,
    key: String,
}

impl Crypto for Vigenere {
    fn encrypt(&self) -> Result<String, super::CryptoError> {
        if self.key.len() < 1 {
            return Err(CryptoError::InvalidKey);
        }

        let mut encripted: String = String::new();

        for i in 0..self.data.len() {
            let index_d = Utils::unicode_to_int(self.data.to_lowercase().chars().nth(i).unwrap());
            let index_k = Utils::unicode_to_int(
                self.key
                    .to_lowercase()
                    .chars()
                    .nth(Utils::modulo(i as i32, self.key.len() as i32) as usize)
                    .unwrap(),
            );

            if index_d < 0 || index_d > 26 {
                return Err(CryptoError::InvalidData);
            } else if index_k < 0 || index_k > 26 {
                return Err(CryptoError::InvalidKey);
            }

            let shifted_index = Utils::modulo(index_d + index_k, ALPHABET_SIZE);

            encripted.push(ALPHABET[shifted_index as usize]);
        }

        Ok(encripted)
    }

    fn decrypt(&self) -> Result<String, super::CryptoError> {
        if self.key.len() < 1 {
            return Err(CryptoError::InvalidKey);
        }

        let mut decripted: String = String::new();

        for i in 0..self.data.len() {
            let index_d = Utils::unicode_to_int(self.data.chars().nth(i).unwrap());
            let index_k = Utils::unicode_to_int(
                self.key
                    .chars()
                    .nth(Utils::modulo(i as i32, self.key.len() as i32) as usize)
                    .unwrap(),
            );

            if index_d < 0 || index_d > 26 {
                return Err(CryptoError::InvalidData);
            } else if index_k < 0 || index_k > 26 {
                return Err(CryptoError::InvalidKey);
            }

            let shifted_index = Utils::modulo(index_d - index_k, ALPHABET_SIZE);

            decripted.push(ALPHABET[shifted_index as usize]);
        }

        Ok(decripted)
    }
}
