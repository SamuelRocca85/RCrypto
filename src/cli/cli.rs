pub use crate::algorithms::{Ceaser, Spartan};
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
    Ceaser(Ceaser),
    Spartan(Spartan),
}
