extern crate clap;

use std::process;
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches};

use crate::action::{ActionParam, Environment};
use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::opltyp::OplCmd;

mod action;
mod config;
mod error;
mod http;
mod opltyp;
mod parse;
mod term;
mod logtyp;

#[tokio::main]
async fn main() -> Result<(), OplError> {
    let action_param = parse_cli().await?;
    let config = create_config().await?;
    action::do_action(action_param, config).await?;
    Ok(())
}

async fn parse_cli() -> Result<ActionParam, OplError> {
    let matches = App::new("tail - following logs made easy!")
        .version("0.1.4")
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
        .subcommand(
            App::new("dqm")
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("environment")
                        .help("[test|prod]")
                        .takes_value(true),
                )
                .about("dqm app")
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

    let mut action_param = ActionParam {
        env: Environment::TEST,
        opltype: OplCmd::CONFIG,
    };

    match matches.subcommand() {
        ("fomis", Some(fomis_matches)) => {
            action_param.env = match_env(fomis_matches);
            match fomis_matches.subcommand() {
                ("list", Some(list_matches)) => {
                    action_param.opltype = OplCmd::FOMIS(match_list(list_matches)?);
                }
                ("config", Some(_config_matches)) => {}
                _ => unreachable!(),
            }
        }
        ("dqm", Some(dqm_matches)) => {
            action_param.env = match_env(dqm_matches);
            match dqm_matches.subcommand() {
                ("list", Some(list_matches)) => {
                    action_param.opltype = OplCmd::DQM(match_list(list_matches)?);
                }
                ("config", Some(_config_matches)) => {}
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    Ok(action_param)
}

fn match_env(arg_matches: &ArgMatches) -> Environment {
    if arg_matches.is_present("env") {
        let opt = Environment::from_str(
            arg_matches
                .value_of("env")
                .expect("Umgebung konnte nicht ausgelesen werden"),
        );
        let env = match opt {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1);
            }
        };

        return env;
    }
    Environment::TEST
}

fn match_list(arg_matches: &ArgMatches) -> Result<Option<u32>, OplError> {
    let day_offset = arg_matches.value_of("day-offset");
    if day_offset.is_some() {
        let result = day_offset
            .unwrap()
            .parse::<u32>()
            .map_err(|err| OplError::new(OplErrorKind::ParseError(err.to_string())))?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
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
