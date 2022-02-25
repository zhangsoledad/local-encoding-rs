//! UTF-8 string converting for non-Windows systems.
use super::Encoder;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Result};

/// Convert UTF-8 bytes to String.
pub struct EncoderUtf8;

impl Encoder for EncoderUtf8 {
    /// Convert UTF-8 to String.
    fn to_string(self: &Self, data: &[u8]) -> Result<String> {
        String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
    }

    /// Convert String to UTF-8.
    fn to_bytes<S: AsRef<OsStr>>(self: &Self, data: S) -> Result<Vec<u8>> {
        Ok(data.as_ref().to_string_lossy().as_bytes().to_vec())
    }
}
