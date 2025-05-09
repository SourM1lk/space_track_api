//! Central error type used across the crate.

use std::fmt;

/// All public errors returned by `spacetrack_rs`.
#[derive(Debug)]
pub enum Error {
    /// HTTP status ≠ 200
    Http(u16),
    /// Failed login (bad credentials)
    Auth,
    /// Response parsed but was empty
    Empty,
    /// Underlying library error
    Ureq(ureq::Error),
    /// JSON (de)serialization failure
    Json(serde_json::Error),
    /// Generic catch-all
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(code) => write!(f, "HTTP error {}", code),
            Error::Auth => write!(f, "authentication failed"),
            Error::Empty => write!(f, "no records returned"),
            Error::Ureq(e) => write!(f, "ureq error: {}", e),
            Error::Json(e) => write!(f, "json error: {}", e),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Error::Ureq(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Other(e.to_string())
    }
}
