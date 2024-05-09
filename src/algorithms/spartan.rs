use super::Crypto;
use crate::algorithms::CryptoError;
use clap::Parser;

type Escitala = Vec<Vec<char>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Spartan {
    data: String,
    key: i32,
}

impl Spartan {
    fn rows(&self) -> i32 {
        let mut rows = self.data.len() as i32 / self.key;
        if self.data.len() as i32 % self.key > 0 {
            rows += 1;
        }
        rows
    }

    fn create_escitala(&self) -> Escitala {
        let rows = self.rows();
        let mut empty_column: Vec<char> = Vec::new();
        let mut escitala: Escitala = vec![];

        for _ in 0..self.key {
            empty_column.push(' ');
        }

        for _ in 0..rows {
            escitala.push(Vec::clone(&empty_column));
        }

        escitala
    }
}

impl Crypto for Spartan {
    fn encrypt(&self) -> Result<String, super::CryptoError> {
        if self.key < 1 {
            return Err(CryptoError::InvalidKey);
        }

        let mut escitala = self.create_escitala();
        let rows = self.rows();
        let mut data_idx: usize = 0;

        for i in 0..rows {
            for j in 0..self.key {
                if data_idx >= self.data.len() {
                    break;
                }
                escitala[i as usize][j as usize] = self
                    .data
                    .to_lowercase()
                    .chars()
                    .nth(data_idx)
                    .expect("Error creando escitala");
                data_idx += 1;
            }
        }

        let mut encripted: String = String::new();
        let rows = self.rows();

        for i in 0..self.key {
            for j in 0..rows {
                if (escitala[j as usize].len() as i32) < i + 1 {
                    break;
                }
                encripted.push(escitala[j as usize][i as usize]);
            }
        }

        Ok(encripted)
    }

    fn decrypt(&self) -> Result<String, super::CryptoError> {
        let mut escitala = self.create_escitala();
        let rows = self.rows();
        let mut data_idx: usize = 0;

        for i in 0..self.key {
            for j in 0..rows {
                if data_idx >= self.data.len() {
                    break;
                }
                escitala[j as usize][i as usize] = self
                    .data
                    .chars()
                    .nth(data_idx)
                    .expect("Error creando escitala");
                data_idx += 1;
            }
        }

        let mut decripted = String::new();
        for i in 0..rows {
            for j in 0..self.key {
                decripted.push(escitala[i as usize][j as usize]);
            }
        }

        Ok(decripted)
    }
}
