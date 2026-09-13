use rayon::prelude::*;

use crate::score::Candidate;
use crate::{Bytes, score::score};
use crate::{Error, Result};

fn candidates(ciphertext: &Bytes) -> impl ParallelIterator<Item = Candidate> + '_ {
    (0..=u8::MAX).into_par_iter().map(|key| {
        let plaintext = ciphertext.single_byte_xor(key);
        let score = score(&plaintext);
        Candidate {
            key,
            score,
            plaintext,
        }
    })
}

pub fn solve_one(ciphertext: &Bytes) -> Result<Candidate> {
    candidates(ciphertext)
        .filter(Candidate::is_plausible)
        .max_by_key(|candidate| candidate.score)
        .ok_or(Error::NoSolution)
}

pub fn solve(ciphertexts: &[Bytes]) -> Result<Vec<Candidate>> {
    let answer: Vec<Candidate> = ciphertexts
        .into_par_iter()
        .filter_map(|ciphertext| solve_one(ciphertext).ok())
        .collect();
    if answer.is_empty() {
        Err(Error::NoSolution)
    } else {
        Ok(answer)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{Bytes, config::init_for_tests};

    #[test]
    fn solve_one_works() {
        init_for_tests();
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let ciphertext = Bytes::from_hex(input).expect("a valid HexString");
        let answer = solve_one(&ciphertext).expect("a valid solution");
        assert_eq!(
            answer.plaintext.as_bytes(),
            b"Cooking MC's like a pound of bacon"
        );
    }
}
