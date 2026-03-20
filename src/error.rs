use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NkfError {
    Io(io::Error),
    Conversion(String),
    UnsupportedEncoding(String),
    InvalidMime(String),
    InvalidArgs(String),
}

impl fmt::Display for NkfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NkfError::Io(e) => write!(f, "I/O error: {e}"),
            NkfError::Conversion(s) => write!(f, "Encoding conversion error: {s}"),
            NkfError::UnsupportedEncoding(s) => write!(f, "Unsupported encoding: {s}"),
            NkfError::InvalidMime(s) => write!(f, "Invalid MIME encoding: {s}"),
            NkfError::InvalidArgs(s) => write!(f, "Invalid arguments: {s}"),
        }
    }
}

impl std::error::Error for NkfError {}

impl From<io::Error> for NkfError {
    fn from(e: io::Error) -> Self {
        NkfError::Io(e)
    }
}

#[cfg(test)]
#[path = "tests/error_tests.rs"]
mod tests;
