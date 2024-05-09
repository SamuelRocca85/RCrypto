use std::fmt::Display;

pub use ceaser::Ceaser;
pub use spartan::Spartan;
pub use vigenere::Vigenere;

pub mod ceaser;
pub mod spartan;
pub mod vigenere;

pub struct Utils;

impl Utils {
    pub fn modulo(a: i32, b: i32) -> i32 {
        ((a % b) + b) % b
    }
    pub fn unicode_to_int(c: char) -> i32 {
        let mut index: i32 = (c as i32) - 97;
        if c == ' ' {
            index = 26;
        }
        index
    }
}

pub trait Crypto {
    fn encrypt(&self) -> Result<String, CryptoError>;
    fn decrypt(&self) -> Result<String, CryptoError>;
}

pub enum CryptoError {
    InvalidKey,
}

impl Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CryptoError::InvalidKey => write!(f, "La llave es incorrecta"),
        }
    }
}
