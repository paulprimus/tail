extern crate clap;
use std::io::Read;
use std::process;
use std::str::FromStr;

use crate::action::{ActionParam, Environment};
use crate::config::Config;
use crate::error::OplError;
use crate::http::HttpData;
use crate::opltyp::OplCmdTyp;
use clap::{App, AppSettings, Arg};

mod action;
mod config;
mod error;
mod http;
mod opltyp;
mod parse;
mod term;

#[tokio::main]
async fn main() -> Result<(), OplError> {
    let actionParam = parseCli().await?;
    let config = create_config().await?;
    action::do_action(actionParam, config).await?;
    Ok(())
}

async fn parseCli() -> Result<ActionParam, OplError> {
    let matches = App::new("tail - following logs made easy!")
        .version("0.1.1")
        .author("Paul Pacher")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("fomis")
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("environment")
                        .help("[test|prod]")
                        .takes_value(true),
                )
                .about("fomis app")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("list").arg(
                        Arg::with_name("day-offset")
                            .short("d")
                            .long("day-offset")
                            .takes_value(true)
                            .help("Listet Logdateien der letzten angeführten Tage"),
                    ),
                )
                .subcommand(App::new("config")),
        )
        .subcommand(App::new("list").help_message("Auflistung aller Services"))
        .get_matches();

    match matches.subcommand_name() {
        Some("list") => {
            print!("- DQM");
            print!("- FOMIS");
        }
        Some("fomis") => {}
        _ => unreachable!(),
    };

    let mut actionParam = ActionParam {
        env: Environment::TEST,
        opltype: OplCmdTyp::CONFIG,
    };

    match matches.subcommand() {
        ("fomis", Some(fomis_matches)) => {
            let mut env = Environment::TEST;
            if fomis_matches.is_present("env") {
                env = Environment::from_str(
                    fomis_matches
                        .value_of("env")
                        .expect("Umgebung konnte nicht ausgelesen werden"),
                )
                .unwrap_or_else(|err| {
                    eprintln!("Es wurde keine valide Umgebung angeführt: {}", err);
                    process::exit(1)
                });
                actionParam.env = env;
            }

            match fomis_matches.subcommand() {
                ("list", Some(serve_matches)) => {
                    let day_offset = serve_matches.value_of("day-offset");
                    let mut offset: u32 = 0;
                    if day_offset.is_some() {
                        offset = day_offset.unwrap().parse::<u32>().unwrap_or_else(|err| {
                            eprintln!("Offset muss eine natürlich Zahl sein: {}", err);
                            process::exit(1);
                        });
                        actionParam.opltype = OplCmdTyp::FOMIS(Some(offset));
                    } else {
                        actionParam.opltype = OplCmdTyp::FOMIS(None);
                    }
                }
                ("config", Some(_config_matches)) => {}
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    Ok(actionParam)
}

async fn create_config() -> Result<Config, OplError> {
    let config = match Config::new() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Die Konfiguration konnte nicht ausgelesen werden: {}", err);
            process::exit(1)
        }
    };
    Ok(config)
}

// async fn print_root(
//     &mut stdout: tokio::io::Stdout,
//     data: HttpData,
//     day_offset: u32,
// ) -> Result<(), OplError> {
//     let ergebnis = parse::parse_root(data)?;
//     term::print_root(stdout, ergebnis, day_offset).await?;
//     Ok(())
// }
