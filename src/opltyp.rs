use crate::error::{OplError, OplErrorKind};
use crate::logtyp::LogTyp;
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OplCmd {
    FOMIS(OplAppCmd),
    DQM(OplAppCmd),
    CONFIG,
    LIST,
}

#[derive(Debug, Clone)]
pub enum OplAppCmd {
    LIST(Option<u32>, LogTyp),
    CONFIG,
}

impl FromStr for OplCmd {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplCmd::FOMIS(OplAppCmd::CONFIG)),
            "DQM" => Ok(OplCmd::DQM(OplAppCmd::CONFIG)),
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
