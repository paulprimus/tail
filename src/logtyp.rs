use crate::error::{OplError, OplErrorKind};
use crate::logtyp::LogTyp::LOG;
use core::fmt;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

impl fmt::Display for LogTyp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LogTyp::LOG => write!(f, "LOG")?,
            LogTyp::ACCESS => write!(f, "ACCESS")?,
            LogTyp::START => write!(f, "START")?,
            LogTyp::ALL => write!(f, "ALL")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogTyp {
    LOG,
    ACCESS,
    START,
    ALL,
}

impl FromStr for LogTyp {
    type Err = OplError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "log" => Ok(LogTyp::LOG),
            "start" => Ok(LogTyp::START),
            "access" => Ok(LogTyp::ACCESS),
            "all" => Ok(LogTyp::ALL),
            _ => Err(OplError::new(OplErrorKind::LogTypNotFoundError(
                String::from("Kein gültiger Logtyp! "),
            ))),
        }
    }
}
