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

fn main() -> cryptopals::Result<()> {
    let args = Args::parse();
    if let Err(err) = cryptopals::config::init(&args.wordlist) {
        eprintln!("error: {err}");
        process::exit(1);
    }

    // reading the ciphertexts
    let content = match fs::read_to_string(&args.ciphertext_file) {
        Ok(content) => content,
        Err(source) => {
            eprintln!(
                "error: {}",
                Error::FileRead {
                    path: args.ciphertext_file,
                    source
                }
            );
            process::exit(1);
        }
    };

    let ciphertexts = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(HexString::from_str)
        // INFO: ? after collect drops the result for every item in vector?
        .collect::<cryptopals::Result<Vec<_>>>()?;

    // run the solver
    match args.mode {
        Mode::SingleByteXor => {
            if let Ok(answers) = single_byte_xor::solve(&ciphertexts) {
                for answer in answers {
                    println!("{answer}")
                }
            }
        }
        Mode::RepeatByteXor => todo!(),
    }
    Ok(())
}
