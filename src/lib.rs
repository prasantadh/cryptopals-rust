#![warn(clippy::pedantic)]
#![allow(missing_docs)]
mod basics;
mod bytes;
pub mod config;
mod error;
mod score;

pub use basics::{repeat_byte_xor, single_byte_xor};
pub use bytes::Bytes;
pub use config::config;
pub use error::Error;
pub use score::{Candidate, score};

pub type Result<T> = core::result::Result<T, Error>;
