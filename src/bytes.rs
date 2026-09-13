use crate::{Error, Result};
use base64::prelude::*;
use core::{clone::Clone, fmt::Display, iter::Iterator};
use std::convert::From;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Clone)]
pub struct Bytes(Box<[u8]>);

impl From<&str> for Bytes {
    fn from(s: &str) -> Self {
        Self(Box::from(s.as_bytes()))
    }
}

impl Bytes {
    pub fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.0)
    }

    pub fn from_b64(content: &str) -> Result<Self> {
        BASE64_STANDARD.decode(content).map_or_else(
            |err| Err(Error::Base64Decode(err)),
            |val| Ok(Self(val.into_boxed_slice())),
        )
    }

    pub fn from_hex(content: &str) -> Result<Self> {
        let value = hex::decode(content)?;
        Ok(Self(value.into_boxed_slice()))
    }

    pub fn fixed_xor(&self, other: &Self) -> Result<Self> {
        if self.len() != other.len() {
            return Err(Error::LengthMismatch);
        }
        let answer: Box<[u8]> = self
            .0
            .iter()
            .zip(other.as_bytes())
            .map(|(a, b)| a ^ b)
            .collect();
        Ok(Self(answer))
    }

    pub fn repeat_xor(&self, other: &Self) -> Self {
        if other.is_empty() {
            return self.clone();
        }
        let answer: Box<[u8]> = self
            .0
            .iter()
            .zip(other.as_bytes().iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect();
        Self(answer)
    }

    pub fn single_byte_xor(&self, key: u8) -> Self {
        Self(self.0.iter().map(|b| b ^ key).collect())
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn parse_hex_lines(content: &str) -> Result<Vec<Self>> {
        content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(Self::from_hex)
            .collect()
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.as_bytes() {
            if c.is_ascii() && !c.is_ascii_control() {
                write!(f, "{}", *c as char)?;
            } else {
                write!(f, "\\x{c:02X}")?;
            }
        }
        Ok(())
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn to_base64_works() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(
            Bytes::from_hex(input)
                .expect("input taken from cryptopals must be valid hex string")
                .to_base64(),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn fixed_xor_works() {
        let s1 = Bytes::from_hex("1c0111001f010100061a024b53535009181c").expect("valid hexstring");
        let s2 = Bytes::from_hex("686974207468652062756c6c277320657965").expect("valid hexstring");
        let result =
            Bytes::from_hex("746865206b696420646f6e277420706c6179").expect("valid hexstring");
        assert_eq!(s1.fixed_xor(&s2).expect("successful xor"), result);
    }

    #[test]
    fn repeat_xor_works() {
        let s1 = Bytes::from(
            "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal",
        );
        let s2 = Bytes::from("ICE");
        assert_eq!(
            s1.repeat_xor(&s2),
            Bytes::from_hex(
                "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            ).unwrap()
        )
    }
}
