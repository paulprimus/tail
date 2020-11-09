use crate::error::{OplError, OplErrorKind};
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OplCmdTyp {
    FOMIS(Option<u32>),
    DQM,
    CONFIG,
}

impl FromStr for OplCmdTyp {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplCmdTyp::FOMIS(None)),
            "DQM" => Ok(OplCmdTyp::DQM),
            _ => Err(OplError::new(OplErrorKind::ParseError)),
        }
    }
}

impl fmt::Display for OplCmdTyp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let result = match self {
            OplCmdTyp::FOMIS(_e) => writeln!(f, "{}", "FOMIS".to_string()),
            OplCmdTyp::DQM => writeln!(f, "{}", "DQM".to_string()),
            _ => writeln!(f, "{}", "sf".to_string()),
        };
        result
    }
}
