use crate::error::{OplError, OplErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone)]
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
