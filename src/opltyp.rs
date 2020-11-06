use crate::error::{OplError, OplErrorKind};
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OplCmdTyp {
    FOMIS(FomisCmdTyp),
    DQM,
    CONFIG
}

pub enum FomisCmdTyp {
    LIST(u32)
}

impl FromStr for OplCmdTyp {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplCmdTyp::FOMIS(FomisCmdTyp::LIST(10))),
            "DQM" => Ok(OplCmdTyp::DQM),
            _ => Err(OplError::new(OplErrorKind::ParseError)),
        }
    }
}

impl fmt::Display for OplCmdTyp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OplCmdTyp::FOMIS(FomisCmdTyp) => writeln!(f, "{}", "FOMIS".to_string()),
            OplCmdTyp::DQM => writeln!(f, "{}", "DQM".to_string()),
            _ => {}
        }
    }
}
