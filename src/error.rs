use thiserror::Error;

#[derive(Error, Debug)]
pub enum StegoError {
    #[error("Encryption failed: {0}")]
    Encryption(String),

    #[error("Decryption failed: {0}")]
    Decryption(String),

    #[error("No hidden data found in input")]
    NoDataFound,

    #[error("Cover text too small: need {needed} bytes capacity, have {available}")]
    InsufficientCover { needed: usize, available: usize },

    #[error("Integrity check failed: data corrupted or modified")]
    IntegrityFailure,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, StegoError>;

