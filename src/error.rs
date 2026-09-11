use core::fmt::Display;
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HexDecode(hex::FromHexError),
    FileRead {
        path: PathBuf,
        source: std::io::Error,
    },
    LengthMismatch,
    ConfigAlreadyInitialized,
    NoSolution,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FileRead { path, source } => {
                write!(f, "could not read {}: {source}", path.display())
            }
            Self::HexDecode(err) => write!(f, "invalid hex: {err}"),
            Self::LengthMismatch => write!(f, "inputs have different lengths"),
            Self::ConfigAlreadyInitialized => write!(f, "config::init() was called more than once"),
            Self::NoSolution => write!(f, "no plausible plaintext found"),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::HexDecode(err)
    }
}
