use crossterm::ErrorKind;
use hyper::http;

use std::{error, fmt, io, result};

//pub type Result<T> = result::Result<T, OplError>;
#[derive(Debug)]
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
        match self.0 {
            OplErrorKind::ParseError => write!(f, "Parse Error"),
            OplErrorKind::FileNotFound => write!(f, "Datei nicht gefunden!"),
            OplErrorKind::HyperError => writeln!(f, "Hyper Fehler!"),
            OplErrorKind::IvalidUri => writeln!(f, "Uri ist nicht valide!"),
            OplErrorKind::CrosstermError => writeln!(f, "Crossterm hat nicht geklappt!"),
            OplErrorKind::Utf8Error => writeln!(f, "UTF-8 Fehler"),
        }
    }
}

impl From<io::Error> for OplError {
    fn from(err: io::Error) -> OplError {
        OplError::new(OplErrorKind::FileNotFound)
    }
}

impl From<http::uri::InvalidUri> for OplError {
    fn from(err: http::uri::InvalidUri) -> OplError {
        OplError::new(OplErrorKind::HyperError)
    }
}

impl From<hyper::Error> for OplError {
    fn from(err: hyper::Error) -> OplError {
        OplError::new(OplErrorKind::HyperError)
    }
}

impl From<crossterm::ErrorKind> for OplError {
    fn from(_: ErrorKind) -> Self {
        OplError::new(OplErrorKind::CrosstermError)
    }
}

#[derive(Debug)]
pub enum OplErrorKind {
    ParseError,
    FileNotFound,
    HyperError,
    IvalidUri,
    CrosstermError,
    Utf8Error,
}
