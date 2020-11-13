use crate::error::{OplError, OplErrorKind};
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OplCmdTyp {
    FOMIS(Option<u32>),
    DQM(Option<u32>),
    CONFIG,
}

impl FromStr for OplCmdTyp {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplCmdTyp::FOMIS(None)),
            "DQM" => Ok(OplCmdTyp::DQM(None)),
            _ => Err(OplError::new(OplErrorKind::ParseError(String::from("OplTyp ist nicht bekannt")))),
        }
    }
}

impl fmt::Display for OplCmdTyp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OplCmdTyp::FOMIS(_offset) => writeln!(f, "{}", "FOMIS".to_string()),
            OplCmdTyp::DQM(_offset) => writeln!(f, "{}", "DQM".to_string()),
            _ => writeln!(f, "{}", "sf".to_string()),
        }
    }
}
