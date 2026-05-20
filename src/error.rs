use derive_more::From;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // Internal errors
    MagicNumber {
        received: String,
    },

    InvalidCrc {
        received: u16,
        expected: u16,
    },

    InvaliHeaderSize {
        received: u8,
    },

    InvalidValue {
        received: u8,
        expected: u8,
    },

    // External errors
    #[from]
    Io(std::io::Error),

    #[from]
    TryFromSliceError(std::array::TryFromSliceError),

    #[from]
    Utf8Error(std::str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MagicNumber { received } => write!(
                f,
                "Mismatched Magic Number, received '{}', expected '.FIT'",
                received
            ),
            Error::InvalidCrc { received, expected } => write!(
                f,
                "Mismatch CRC, received '{:X}', expected '{:X}'",
                received, expected
            ),
            Error::InvaliHeaderSize { received } => write!(
                f,
                "Invalid header size, received '{}', expected '14'",
                received
            ),
            _ => write!(f, "self:#?"),
        }
    }
}

impl std::error::Error for Error {}
