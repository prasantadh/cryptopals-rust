use core::{clone::Clone, fmt::Display, result::Result::Ok, str::FromStr};

use crate::{Error, Result};
use base64::prelude::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct HexString(Vec<u8>);

impl FromStr for HexString {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value = hex::decode(s)?;
        Ok(Self(value))
    }
}

// INFO: it might not be a great idea to implement From<[u8]>
// because we would be unsure if it is a hex string b"ae28ff"
// or if it is an actual values we should hold

impl HexString {
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.0)
    }

    pub fn fixed_xor(&self, other: &HexString) -> Result<Vec<u8>> {
        if self.0.len() != other.len() {
            return Err(Error::LengthMismatch);
        }
        let mut answer = vec![0; self.len()];
        for i in 0..self.len() {
            answer[i] = self.0[i] ^ other.nth(i);
        }
        Ok(answer)
    }

    pub fn single_byte_xor(&self, key: u8) -> HexString {
        let mut ciphertext = self.clone();
        for i in 0..self.len() {
            ciphertext.0[i] ^= key;
        }
        ciphertext
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn nth(&self, i: usize) -> u8 {
        self.0[i]
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Display for HexString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.as_bytes() {
            if c.is_ascii() && !c.is_ascii_control() {
                write!(f, "{}", *c as char)?
            } else {
                write!(f, "\\x{:02X}", c)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use crate::HexString;
    #[test]
    fn to_base64_works() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            HexString::from_str(input)
                .expect("input taken from cryptopals must be valid hex string")
                .to_base64(),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        )
    }

    #[test]
    fn fixed_xor_works() {
        let s1 =
            HexString::from_str("1c0111001f010100061a024b53535009181c").expect("valid hexstring");
        let s2 =
            HexString::from_str("686974207468652062756c6c277320657965").expect("valid hexstring");
        let result =
            HexString::from_str("746865206b696420646f6e277420706c6179").expect("valid hexstring");
        assert_eq!(
            s1.fixed_xor(&s2).expect("successful xor"),
            result.as_bytes()
        );
    }
}
