use crate::error::{OplError, OplErrorKind};
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OplCmd {
    FOMIS(Option<u32>),
    DQM(OplAppCmd),
    CONFIG,
    LIST,
}

pub enum OplAppCmd {
    LIST(Option<u32>),
    CONFIG,
}

impl FromStr for OplCmd {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplCmd::FOMIS(None)),
            "DQM" => Ok(OplCmd::DQM(None)),
            _ => Err(OplError::new(OplErrorKind::ParseError(String::from(
                "OplTyp ist nicht bekannt",
            )))),
        }
    }
}

impl fmt::Display for OplCmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OplCmd::FOMIS(_offset) => writeln!(f, "{}", "FOMIS".to_string()),
            OplCmd::DQM(_offset) => writeln!(f, "{}", "DQM".to_string()),
            _ => writeln!(f, "{}", "sf".to_string()),
        }
    }
}
