use core::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HexDecode(hex::FromHexError),
    LengthMismatch,
    ConfigAlreadyInitialized,
    NoSolution,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::HexDecode(err)
    }
}
