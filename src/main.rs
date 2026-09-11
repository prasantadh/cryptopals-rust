use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use cryptopals::{HexString, detect_single_byte_xor, single_byte_xor};
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
    DetectSingleByteXor,
    RepeatByteXor,
}

fn main() {
    let args = Args::parse();
    cryptopals::config::init(&args.wordlist).expect("a correct wordlist file");

    // reading the ciphertexts
    let content =
        fs::read_to_string(args.ciphertext_file).expect("failed to read the wordlist file");
    let ciphertexts: Vec<HexString> = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            HexString::try_from(String::from(line.trim()))
                .expect("a valid hexstring for ciphertext")
        })
        .collect();

    // run the solver
    match args.mode {
        Mode::SingleByteXor => {
            // each solver probably expects a different format of the file so might be fine to read
            // the file here and pass the input
            let answer = single_byte_xor::solve(&ciphertexts[0]);
            println!("{:?}", answer);
        }
        Mode::DetectSingleByteXor => {
            if let Ok(answers) = detect_single_byte_xor::solve(&ciphertexts) {
                for answer in answers {
                    println!("{answer}")
                }
            }
        }
        Mode::RepeatByteXor => todo!(),
    }
}
