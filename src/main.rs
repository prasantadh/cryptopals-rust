use core::str::FromStr;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use cryptopals::{HexString, single_byte_xor};
use std::fs;

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

fn main() {
    let args = Args::parse();
    cryptopals::config::init(&args.wordlist).expect("a correct wordlist file");

    // reading the ciphertexts
    // INFO: expect won't produce a very nice message, just crash
    let content =
        fs::read_to_string(args.ciphertext_file).expect("failed to read the wordlist file");
    let ciphertexts: Vec<HexString> = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| HexString::from_str(line.trim()).expect("a valid hexstring for ciphertext"))
        .collect();

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
}
