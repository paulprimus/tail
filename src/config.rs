use serde::Deserialize;

// use crate::action::Environment::TEST;
use crate::action::{ActionParam, Environment};
use crate::error::{OplError, OplErrorKind};
use crate::oplapp::Oplapp;
use crate::oplcmd::OplCmd;
use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub fomis: Fomis,
    pub dqm: Dqm,
}

pub trait PrintableApp {
    fn stringify(&self) -> String;
}

#[derive(Debug, Deserialize)]
pub struct Fomis {
    root: Root,
}

#[derive(Debug, Deserialize)]
pub struct Dqm {
    root: Root,
}

impl PrintableApp for Dqm {
    fn stringify(&self) -> String {
        let mut v = String::from("Prod: ");
        v.push_str(&self.root.prod);
        v.push('\n');
        v.push_str("Test: ");
        v.push_str(&self.root.test);
        v
    }
}

impl PrintableApp for Fomis {
    fn stringify(&self) -> String {
        let mut v = String::from("Prod: ");
        v.push_str(&self.root.prod);
        v.push('\n');
        v.push_str("Test: ");
        v.push_str(&self.root.test);
        v
    }
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
    pub fn get_url_for(&self, oplApp: &Oplapp) -> Result<String, OplError> {
        let url: String = match oplApp {
            Oplapp::FOMIS => {
                if action_param.env == Environment::TEST {
                    self.fomis.root.test.to_string()
                } else {
                    self.fomis.root.prod.to_string()
                }
            }
            Oplapp::DQM => {
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
