use crate::error::{OplError, OplErrorKind};
use crate::logtyp::LogTyp;
use serde::export::Formatter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum OplCmd {
    FOMIS(OplAppCmd),
    DQM(OplAppCmd),
    CONFIG,
    LIST,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OplAppCmd {
    LIST(Option<u32>, LogTyp, bool),
    CONFIG,
}

impl FromStr for OplCmd {
    type Err = OplError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FOMIS" => Ok(OplCmd::FOMIS(OplAppCmd::CONFIG)),
            "DQM" => Ok(OplCmd::DQM(OplAppCmd::CONFIG)),
            _ => Err(OplError::new(OplErrorKind::ParseError(String::from(
                "OplCmd ist nicht bekannt",
            )))),
        }
    }
}

impl fmt::Display for OplCmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::FOMIS(cmd) => writeln!(f, "{} {}", "FOMIS: ".to_string(), cmd),
            Self::DQM(cmd) => writeln!(f, "{} {}", "DQM: ".to_string(), cmd),
            _ => writeln!(f, "{}", "sf".to_string()),
        }
    }
}

impl fmt::Display for OplAppCmd {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            OplAppCmd::LIST(o, logtyp, b) => {
                let offset = o.map_or(String::from("No offset"), |d| d.to_string());
                writeln!(f, "{} - {} - {}", offset, logtyp, b)
            }
            OplAppCmd::CONFIG => writeln!(f, "{}", "test"),
        }
    }
}
