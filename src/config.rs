use serde::Deserialize;

// use crate::action::Environment::TEST;
use crate::action::{ActionParam, Environment};
use crate::error::{OplError, OplErrorKind};
use crate::opltyp::OplCmd;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Display;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub fomis: Fomis,
    pub dqm: Dqm,
}

trait Appl {
    get_config
}

#[derive(Debug, Deserialize)]
struct Fomis {
    root: Root,
}

#[derive(Debug, Deserialize)]
struct Dqm {
    root: Root,
}

#[derive(Debug, Deserialize)]
struct Root {
    test: String,
    prod: String,
}

fn parse() -> Result<Config, OplError> {
    let mut inhalt_config = String::new();
    File::open("config.toml").and_then(|mut f| f.read_to_string(&mut inhalt_config))?;
    let config: Config = toml::from_str(&inhalt_config)
        .map_err(|err| OplError::new(OplErrorKind::ParseError(err.to_string())))?;
    Ok(config)
}

impl Config {
    pub fn get_url_for(&self, action_param: &ActionParam) -> Result<String, OplError> {
        let url: String = match action_param.oplcmd {
            OplCmd::FOMIS(_offset) => {
                if action_param.env == Environment::TEST {
                    self.fomis.root.test.to_string()
                } else {
                    self.fomis.root.prod.to_string()
                }
            }
            OplCmd::DQM(_offset) => {
                if action_param.env == Environment::TEST {
                    self.dqm.root.test.to_string()
                } else {
                    self.dqm.root.prod.to_string()
                }
            }
            _ => unreachable!(),
        };
        Ok(url)
    }

    pub fn new() -> Result<Config, OplError> {
        let config = parse()?;
        Ok(config)
    }

    pub fn getConfigFor() {}
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DQM{}\nFOMIS{}\n", self.dqm, self.fomis)
    }
}

impl fmt::Display for Dqm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl fmt::Display for Fomis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nTest: {}\nProd: {}", self.test, self.prod)
    }
}
