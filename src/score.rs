use crate::{HexString, config};

pub fn score(text: &HexString) -> u64 {
    // also need to score based on the words
    // oncecell with a wordlist, then consult that in the future
    let words = text.as_bytes().split(|b| *b == b' ');
    let mut answer = 0;
    for word in words {
        answer += if config().wordlist.contains(&word.to_ascii_lowercase()) {
            word.len() * word.len()
        } else {
            0
        }
    }
    answer as u64
}
