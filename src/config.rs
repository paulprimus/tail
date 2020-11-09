use serde::Deserialize;

use crate::action::Environment;
use crate::error::{OplError, OplErrorKind};
use crate::opltyp::OplCmdTyp;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    fomis: Fomis,
    dqm: Dqm,
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
    let mut config: Config =
        toml::from_str(&inhalt_config).map_err(|_| OplError::new(OplErrorKind::ParseError))?;
    Ok(config)
}

impl Config {
    pub fn get_url_for(&self, opl_typ: OplCmdTyp) -> Result<String, OplError> {
        //let config = parse()?;
        let url: String = match opl_typ {
            OplCmdTyp::FOMIS(_fc) => self.fomis.root.test.to_string(),
            OplCmdTyp::DQM => self.dqm.root.test.to_string(),
            _ => unreachable!(),
        };
        Ok(url)
    }
    pub fn get_config_for(self, opl_typ: OplCmdTyp, env: Environment) -> Result<String, OplError> {
        //let config = parse()?;
        let url = match opl_typ {
            OplCmdTyp::FOMIS(fomisCmdTyp) => self.fomis.root.test,
            OplCmdTyp::DQM => self.dqm.root.test,
            _ => String::from(""),
        };
        Ok(url)
    }

    pub fn new() -> Result<Config, OplError> {
        let mut config = parse()?;
        Ok(config)
    }
}
