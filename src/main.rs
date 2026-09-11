use cryptopals::{Error, HexString, single_byte_xor};

use clap::{Parser, ValueEnum};
use core::str::FromStr;
use std::fs;
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    ciphertext_file: PathBuf,
    #[arg(short, long)]
    wordlist: PathBuf,
    #[arg(short, long, value_enum)]
    mode: Mode,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
enum Mode {
    SingleByteXor,
    RepeatByteXor,
}

fn run() -> cryptopals::Result<()> {
    let args = Args::parse();
    cryptopals::config::init(&args.wordlist)?;

    // reading the ciphertexts
    let content = fs::read_to_string(&args.ciphertext_file).map_err(|source| Error::FileRead {
        path: args.ciphertext_file,
        source,
    })?;

    let ciphertexts = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(HexString::from_str)
        .collect::<cryptopals::Result<Vec<_>>>()?;

    // run the solver
    match args.mode {
        Mode::SingleByteXor => {
            for answer in single_byte_xor::solve(&ciphertexts)? {
                println!("{answer}")
            }
        }
        Mode::RepeatByteXor => todo!(),
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
