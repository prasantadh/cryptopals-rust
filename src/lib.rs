mod basics;
pub mod config;
mod error;
mod hexstring;
mod score;

pub use basics::{detect_single_byte_xor, single_byte_xor};
pub use config::config;
pub use error::Error;
pub use error::Result;
pub use hexstring::HexString;
pub use score::score;

pub type Ciphertexts = Vec<HexString>;
