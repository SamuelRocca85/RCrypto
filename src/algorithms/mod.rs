use std::fmt::Display;

pub use caesar::Caesar;
pub use spartan::Spartan;
pub use vigenere::Vigenere;

pub mod caesar;
pub mod spartan;
pub mod vigenere;

pub static ALPHABET_SIZE: i32 = 27;

pub static ALPHABET: [char; 27] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ',
];

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
    InvalidData,
}

impl Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CryptoError::InvalidKey => write!(f, "La llave es incorrecta"),
            CryptoError::InvalidData => write!(
                f,
                "La data contiene caracteres invalidos, use unicamente A-Z o espacios"
            ),
        }
    }
}
