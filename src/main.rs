mod config;
mod error;
mod http;
mod opltyp;
mod term;

extern crate clap;
extern crate crossterm;
//#[macro_use]
//extern crate lazy_static;
//extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::io;

use std::time::Duration;

use clap::{App, AppSettings, Arg};

use hyper::{body::HttpBody, client::HttpConnector};
use hyper_tls::HttpsConnector;
use tokio::signal;
use tokio::time::{self};

use crossterm::terminal::ClearType;
use std::str::FromStr;

use crate::config::Config;
use crate::error::OplError;
use crate::http::{fetch_url, HttpData};
use crate::opltyp::OplTyp;
use crate::term::enter_alternate_screen;

#[tokio::main]
async fn main() -> Result<(), OplError> {
    let matches = App::new("tail - following logs made easy!")
        .version("0.1.1")
        .author("Paul Pacher")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("fomis")
                .about("fomis app")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("serve"))
                .subcommand(App::new("config")),
        )
        .subcommand(App::new("list").help_message("Auflistung aller Services"))
        .get_matches();

    let config: Config = config::parse().expect("Fehler beim Parsen der Konfigurationsdatei!");
    match matches.subcommand_name() {
        Some("list") => {
            print!("- {}", OplTyp::DQM);
            print!("- {}", OplTyp::FOMIS);
        }
        Some("fomis") => { /*Nichts*/ }
        _ => unreachable!(),
    };

    let out = io::stdout();
    let mut out_locked = out.lock();

    match matches.subcommand() {
        ("fomis", Some(fomis_matches)) => match fomis_matches.subcommand_name() {
            Some("serve") => {
                if let Some(mut data) = fetch_url(OplTyp::FOMIS, &config).await? {
                    //enter_alternate_screen(&mut out_locked, &mut data)?;
                    config.print_root(&mut out_locked, &mut data, OplTyp::FOMIS);
                }
            }
            Some("config") => println!("{}", config.get_config_for(OplTyp::FOMIS)?),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
