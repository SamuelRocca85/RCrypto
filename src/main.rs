use std::process::ExitCode;

use algorithms::CryptoError;
use clap::Parser;
use cli::{Algorithms, Args};

use crate::algorithms::Crypto;

mod algorithms;
mod cli;

fn main() -> ExitCode {
    let args = Args::parse();

    match &args.algorithm {
        Algorithms::Caesar(algo) => {
            let data: Result<String, CryptoError>;

            if args.encrypt {
                data = algo.encrypt();
            } else if args.decrypt {
                data = algo.decrypt();
            } else {
                println!("No se especifico un modo de encripcion");
                return ExitCode::FAILURE;
            }

            match data {
                Ok(message) => {
                    println!("{}", message);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    println!("Error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Algorithms::Spartan(algo) => {
            let data: Result<String, CryptoError>;

            if args.encrypt {
                data = algo.encrypt();
            } else if args.decrypt {
                data = algo.decrypt();
            } else {
                println!("No se especifico un modo de encripcion");
                return ExitCode::FAILURE;
            }

            match data {
                Ok(message) => {
                    println!("{}", message);
                    return ExitCode::SUCCESS;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
        Algorithms::Vigenere(algo) => {
            let data: Result<String, CryptoError>;

            if args.encrypt {
                data = algo.encrypt();
            } else if args.decrypt {
                data = algo.decrypt();
            } else {
                println!("No se especifico un modo de encripcion");
                return ExitCode::FAILURE;
            }

            match data {
                Ok(message) => {
                    println!("{}", message);
                    return ExitCode::SUCCESS;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
    }
}
