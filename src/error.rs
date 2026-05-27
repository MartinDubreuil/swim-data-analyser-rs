use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // Internal errors
    #[error("Mismatched Magic Number, received '{received}', expected '.FIT'")]
    MagicNumber { received: String },

    #[error("Mismatch CRC, received '{received:X}', expected '{expected:X}'")]
    InvalidCrc { received: u16, expected: u16 },

    #[error("Invalid header size, received '{received}', expected '14'")]
    InvaliHeaderSize { received: u8 },

    #[error("Invalid value, received '{received}', expected '{expected}'")]
    InvalidValue { received: u8, expected: u8 },

    // External errors
    #[error("transparent")]
    Io(#[from] std::io::Error),

    #[error("transparent")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),

    #[error("transparent")]
    Utf8Error(#[from] std::str::Utf8Error),
}
