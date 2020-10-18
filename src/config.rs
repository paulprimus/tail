use serde::Deserialize;

use crate::error::{OplError, OplErrorKind};
use crate::http::HttpData;
use crate::opltyp::OplTyp;
use crate::parse;
use crate::term;
use std::fs::File;
use std::io::{Read, StdoutLock};
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

// impl Fomis {
//     fn url(&self) -> (String,String) {
//         (&self.root.test,&self.root.prod);
//     }
// }

#[derive(Debug, Deserialize)]
struct Dqm {
    root: Root,
}

#[derive(Debug, Deserialize)]
struct Root {
    test: String,
    prod: String,
}

pub fn parse() -> Result<Config, OplError> {
    let mut inhalt_config = String::new();
    File::open("config.toml").and_then(|mut f| f.read_to_string(&mut inhalt_config))?;
    let config: Config =
        toml::from_str(&inhalt_config).map_err(|_| OplError::new(OplErrorKind::ParseError))?;
    Ok(config)
}

impl Config {
    pub fn get_url_for(&self, opl_typ: OplTyp) -> Result<String, OplError> {
        //let config = parse()?;
        let url: String = match opl_typ {
            OplTyp::FOMIS => self.fomis.root.test.to_string(),
            OplTyp::DQM => self.dqm.root.test.to_string(),
            _ => unreachable!(),
        };
        Ok(url)
    }
    pub fn get_config_for(self, opl_typ: OplTyp) -> Result<String, OplError> {
        //let config = parse()?;
        let url = match opl_typ {
            OplTyp::FOMIS => self.fomis.root.test,
            OplTyp::DQM => self.dqm.root.test,
            _ => unreachable!(),
        };
        Ok(url)
    }
}
