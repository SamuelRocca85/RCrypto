use super::{Crypto, CryptoError};

pub struct Vigenere {
    data: String,
    key: String,
}

impl Crypto for Vigenere {
    fn encrypt(&self) -> Result<String, super::CryptoError> {
        if self.key.len() < 1 {
            return Err(CryptoError::InvalidKey);
        }

        Ok("".to_string())
    }

    fn decrypt(&self) -> Result<String, super::CryptoError> {
        todo!()
    }
}
