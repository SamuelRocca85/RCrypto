use super::{Crypto, Utils};
use crate::algorithms::CryptoError;
use clap::Parser;

static ALPHABET_SIZE: i32 = 27;

static ALPHABET: [char; 27] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ',
];

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
            // let mut index: i32 = (c as i32) - 97;
            // if c == ' ' {
            //     index = 26;
            // }
            let index: i32 = Utils::unicode_to_int(c);

            // let new_index = (index + self.key) % ALPHABET_SIZE;
            let new_index = Utils::modulo(index + self.key, ALPHABET_SIZE);

            encripted.push(ALPHABET[new_index as usize])
        }

        Ok(encripted)
    }

    fn decrypt(&self) -> Result<String, CryptoError> {
        if self.key < 0 {
            return Err(CryptoError::InvalidKey);
        }

        let mut decripted: String = String::new();

        for c in self.data.to_lowercase().chars() {
            // let mut index: i32 = (c as i32) - 97;
            // if c == ' ' {
            //     index = 26;
            // }
            let index: i32 = Utils::unicode_to_int(c);
            // let new_index = (((index - self.key) % ALPHABET_SIZE) + ALPHABET_SIZE) % ALPHABET_SIZE;
            let new_index = Utils::modulo(index - self.key, ALPHABET_SIZE);

            decripted.push(ALPHABET[new_index as usize])
        }

        Ok(decripted)
    }
}
