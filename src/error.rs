//use crossterm::ErrorKind;
use hyper::http;

use std::{error, fmt, io};

#[derive(Debug, PartialEq)]
pub struct OplError(OplErrorKind);

impl error::Error for OplError {}

impl OplError {
    /// A crate private constructor for `Error`.
    pub fn new(kind: OplErrorKind) -> OplError {
        OplError(kind)
    }
}

impl fmt::Display for OplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            OplErrorKind::ParseError => write!(f, "Parse Error"),
            OplErrorKind::FileNotFound(err) => write!(f, "Datei nicht gefunden! {}", err),
            OplErrorKind::HyperError(err) => writeln!(f, "Hyper Fehler: {}", err),
            OplErrorKind::IvalidUri => writeln!(f, "Uri ist nicht valide!"),
            // OplErrorKind::CrosstermError => writeln!(f, "Crossterm hat nicht geklappt!"),
            OplErrorKind::Utf8Error => writeln!(f, "UTF-8 Fehler"),
        }
    }
}

impl From<io::Error> for OplError {
    fn from(err: io::Error) -> OplError {
        OplError::new(OplErrorKind::FileNotFound(err.to_string()))
    }
}

impl From<http::uri::InvalidUri> for OplError {
    fn from(err: http::uri::InvalidUri) -> OplError {
        OplError::new(OplErrorKind::HyperError(err.to_string()))
    }
}

impl From<hyper::Error> for OplError {
    fn from(err: hyper::Error) -> OplError {
        OplError::new(OplErrorKind::HyperError(err.to_string()))
    }
}

#[derive(Debug, PartialEq)]
pub enum OplErrorKind {
    ParseError,
    FileNotFound(String),
    HyperError(String),
    IvalidUri,
    // CrosstermError,
    Utf8Error,
}
