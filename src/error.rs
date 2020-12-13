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
            OplErrorKind::ParseError(err) => write!(f, "Parsing Error: {}", err),
            OplErrorKind::FileNotFound(err) => write!(f, "Datei nicht gefunden! {}", err),
            OplErrorKind::HyperError(err) => writeln!(f, "Hyper Fehler: {}", err),
            //    OplErrorKind::IvalidUri => writeln!(f, "Uri ist nicht valide!"),
            OplErrorKind::EnvironmentNotFoundError => writeln!(
                f,
                "Die angeführte Umgebung existiert nicht! Erlaubt sind: [test/prod]"
            ),
            OplErrorKind::LogTypNotFoundError(err) => writeln!(
                f,
                "Logtyp nicht vorhanden! Erlaubt sind: log|start|access \n{}",
                err
            ),
            OplErrorKind::RootLogError => writeln!(f, "Fehler beim sortieren der Rootlog-Dateien nach Datum!")
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
    ParseError(String),
    FileNotFound(String),
    HyperError(String),
    RootLogError,
    EnvironmentNotFoundError,
    LogTypNotFoundError(String), // Utf8Error,
}
