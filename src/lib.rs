//! A small utility crate for parsing solana logs
pub use error::LogParseError;
pub use parsed_log::ParsedLog;
pub use raw_log::RawLog;
pub use structured_log::{parsed::ParsedStructuredLog, raw::RawStructuredLog};

pub mod error;
pub mod parsed_log;
pub mod raw_log;
pub mod structured_log;

pub type Result<T> = std::result::Result<T, LogParseError>;

/// A small utility function to check if a string is a valid Solana public key.
pub fn quick_pubkey_check(pubkey: &str) -> bool {
    const MIN_CHARS: usize = 32;
    const MAX_CHARS: usize = 44;

    // pubkey should be composed of ascii characters
    let bytes = pubkey.as_bytes();
    let len = bytes.len();

    // Check if the length is within the valid range
    #[allow(clippy::manual_range_contains)]
    if len < MIN_CHARS || len > MAX_CHARS {
        return false;
    }

    // Check characters are valid base58 characters
    bytes.iter().all(|b| {
        matches!(
            b,
            b'1'..=b'9' | b'A'..=b'H' | b'J'..=b'N' | b'P'..=b'Z' | b'a'..=b'k' | b'm'..=b'z'
        )
    })
}
