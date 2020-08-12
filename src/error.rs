use std::{error, fmt, result, io};

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
        }
    }
}

impl From<io::Error> for OplError {
    fn from(err: io::Error) -> OplError {
        OplError::new(OplErrorKind::FileNotFound)
    }
}

#[derive(Debug)]
pub enum OplErrorKind {
    ParseError,
    FileNotFound,
}
