use serde::Deserialize;

use crate::opl_typ::OplTyp;
use crate::error::{OplError, OplErrorKind};
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
    root: Root
}

#[derive(Debug, Deserialize)]
struct Root {
    test: String,
    prod: String
}

fn parse() -> Result<Config, OplError> {
    let mut inhalt_config = String::new();
    File::open("config.toml")
        .and_then(|mut f| f.read_to_string(&mut inhalt_config))?;
    let config: Config = toml::from_str(&inhalt_config).map_err(|_| OplError::new(OplErrorKind::ParseError))?;
    Ok(config)
}

pub fn print_config(opl_typ: OplTyp) -> Result<(), OplError>{
    let config = parse()?;
    match opl_typ {
        OplTyp::FOMIS => println!("{:?}", config.fomis.root),
        OplTyp::DQM => println!("{:?}", config.dqm.root),
        _ => print!("{:?}", config.dqm)
    }
    Ok(())
}
