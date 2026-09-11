use core::{iter::Iterator, result::Result::Err};

use crate::{Error, HexString, Result, single_byte_xor};

pub fn solve(ciphertexts: &[HexString]) -> Result<Vec<HexString>> {
    let answer: Vec<HexString> = ciphertexts
        .iter()
        .filter_map(|ciphertext| single_byte_xor::solve(ciphertext).ok())
        .collect();
    if answer.len() == 0 {
        Err(Error::NoSolution)
    } else {
        Ok(answer)
    }
}
