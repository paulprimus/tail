use crate::error::{OplError, OplErrorKind};
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OplTyp {
    FOMIS,
    DQM,
}

impl FromStr for OplTyp {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplTyp::FOMIS),
            "DQM" => Ok(OplTyp::DQM),
            _ => Err(OplError::new(OplErrorKind::ParseError)),
        }
    }
}

impl fmt::Display for OplTyp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OplTyp::FOMIS => writeln!(f, "{}", "FOMIS".to_string()),
            OplTyp::DQM => writeln!(f, "{}", "DQM".to_string()),
        }
    }
}
