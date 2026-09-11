use core::cmp::Ord;
use rayon::prelude::*;

use crate::{Error, Result};
use crate::{HexString, score::score};

pub fn solve_one(ciphertext: &HexString) -> Result<HexString> {
    let answer = (0..=255)
        .into_par_iter()
        .map(|key| {
            let answer = ciphertext.single_byte_xor(key);
            let answer_score = score(&answer);
            (answer_score, answer)
        })
        .max_by(|&(k1, _), &(k2, _)| k1.cmp(&k2))
        .unwrap();
    // INFO: This is a bit of a hard-coded threshold
    // which check for at least one valid word with 3 characters
    // in it. works for our current scoring algorithm but would
    // likely change based on how we score in the future
    if answer.0 < 10 {
        Err(Error::NoSolution)
    } else {
        Ok(answer.1)
    }
}

pub fn solve(ciphertexts: &[HexString]) -> Result<Vec<HexString>> {
    let answer: Vec<HexString> = ciphertexts
        .par_iter()
        .filter_map(|ciphertext| solve_one(ciphertext).ok())
        .collect();
    if answer.len() == 0 {
        Err(Error::NoSolution)
    } else {
        Ok(answer)
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;
    use std::path::Path;

    use super::*;
    use crate::HexString;

    #[test]
    fn solve_one_works() {
        // INFO: this test depends on where the wordlist being available
        // turn this into using a local fixture
        crate::config::init(Path::new("/usr/share/dict/words")).expect("a valid wordlist path");
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext = HexString::from_str(input).expect("a valid HexString");
        let answer = solve_one(&ciphertext).expect("a valid solution");
        assert_eq!(answer.as_bytes(), b"Cooking MC's like a pound of bacon");
    }
}
