pub use crate::algorithms::{Caesar, Spartan, Vigenere};
pub use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub algorithm: Algorithms,

    #[arg(short, long)]
    pub encrypt: bool,

    #[arg(short, long)]
    pub decrypt: bool,
}

#[derive(Subcommand, Debug)]
pub enum Algorithms {
    Caesar(Caesar),
    Spartan(Spartan),
    Vigenere(Vigenere),
}
