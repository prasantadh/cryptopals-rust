use crate::{Bytes, config};

const MIN_SCORE: u64 = 10;

#[derive(Debug)]
// INFO: Is this okay to make the fields public?
pub struct Candidate {
    pub key: u8,
    pub score: u64,
    // INFO: is it better to hold a reference to ciphertext
    // and perhaps a decrypt function rather have a bunch of plaintext
    // is that better for memory and/or a better approach in general?
    pub plaintext: Bytes,
}

impl Candidate {
    pub const fn is_plausible(&self) -> bool {
        self.score >= MIN_SCORE
    }
}

pub fn score(text: &Bytes) -> u64 {
    let words = text.as_bytes().split(|b| *b == b' ');
    let mut answer = 0;
    for word in words {
        answer += if config()
            .wordlist
            .contains(word.to_ascii_lowercase().as_slice())
        {
            word.len() * word.len()
        } else {
            0
        }
    }
    answer as u64
}
