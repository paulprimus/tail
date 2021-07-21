use crate::error::{OplError, OplErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Oplapp {
    DQM,
    FOMIS,
}

impl FromStr for Oplapp {
    type Err = OplError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "fomis" => Ok(Oplapp::FOMIS),
            "dqm" => Ok(Oplapp::DQM),
            _ => Err(OplError::new(OplErrorKind::EnvironmentNotFoundError)),
        }
    }
}
