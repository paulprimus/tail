extern crate clap;
extern crate hyper;

use clap::{App, AppSettings, Arg, ArgMatches};
use std::process;
use std::str::FromStr;

use crate::action::{ActionParam, Environment};
use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::logtyp::LogTyp;
use crate::oplapp::Oplapp;
use crate::oplcmd::{ListCmd, OplCmd};

mod action;
mod applog;
mod config;
mod error;
mod http;
mod logtyp;
mod oplapp;
mod oplcmd;
mod opldate;
mod rootlogs;
mod term;

#[tokio::main]
async fn main() -> Result<(), OplError> {
    let action_param = parse_cli().await?;
    let config = create_config().await?;
    action::dispatch(action_param, config).await?;
    Ok(())
}

async fn parse_cli() -> Result<ActionParam, OplError> {
    let matches = App::new("tail - following logs made easy!")
        .version("0.1.5")
        .author("Paul Pacher")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("list")
                .arg(Arg::with_name("app")
                    .short("a")
                    .long("app")
                    .takes_value(true)
                    .required(true)
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("environment")
                        .help("[test|prod]")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("day-offset")
                        .short("d")
                        .long("day-offset")
                        .takes_value(true)
                        .help("Listet Logdateien der letzten angeführten Tage"),
                )
                .arg(
                    Arg::with_name("log-typ")
                        .short("t")
                        .long("typ")
                        .takes_value(true)
                        .help("start|log|access"),
                )
                .arg(Arg::with_name("fetch")
                    .short("f")
                    .long("fetch")
                    .takes_value(false)
                    .help("Daten werden von Webserver geholt. Wird dieses Argument nicht angeführt wird noch einem lokalen Json-File gesucht.")
                )
                .about("fomis app")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                // .subcommand(
                //     App::new("list")
                //    
                // )
               // .subcommand(App::new("config")),
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
                    App::new("list")
                        .arg(
                            Arg::with_name("day-offset")
                                .short("d")
                                .long("day-offset")
                                .takes_value(true)
                                .help("Listet Logdateien der letzten angeführten Tage")
                        )
                        .arg(
                            Arg::with_name("log-typ")
                                .short("t")
                                .long("typ")
                                .takes_value(true)
                                .help("start|log|access")
                        )
                        .arg(Arg::with_name("fetch")
                            .short("f")
                            .long("fetch")
                            .takes_value(false)
                            .help("Daten werden von Webserver geholt. Wird dieses Argument nicht angeführt wird noch einem lokalen Json-File gesucht.")
                        )
                    ,
                )
                .subcommand(App::new("config")),
        )
        .subcommand(App::new("list").help_message("Auflistung aller Services"))
        .get_matches();

    let mut action_param = ActionParam {
        env: Environment::TEST,
        oplcmd: OplCmd::CONFIG,
    };

    match matches.subcommand() {
        ("list", Some(matches)) => {
            // action_param.env = match_list(matches);
            let (env, offset, logtyp1, fetch) = match_list(matches)?;
            let listCmd = ListCmd {
                app: Oplapp::FOMIS,
                offset: offset,
                logtyp: logtyp1,
                fetch: fetch,
            };
            action_param.oplcmd = OplCmd::LIST(listCmd);
            action_param.env = env;

            // match fomis_matches.subcommand() {
            //     ("list", Some(list_matches)) => {
            //         let x = match_list(list_matches)?;
            //         action_param.oplcmd = OplCmd::FOMIS(OplAppCmd::LIST(x.0, x.1, x.2));
            //     }
            //     ("config", Some(_config_matches)) => {}
            //     _ => unreachable!(),
            // }
        }
        ("dqm", Some(dqm_matches)) => {
            action_param.env = match_env(dqm_matches);
            match dqm_matches.subcommand() {
                ("list", Some(list_matches)) => {
                    let x = match_list(list_matches)?;
                }
                // ("config", Some(_config_matches)) => {
                //     action_param.oplcmd = OplCmd::DQM(OplAppCmd::CONFIG)
                // }
                _ => unreachable!(),
            }
        }
        // ("list", Some(_list_matches)) => {
        //     action_param.oplcmd = OplCmd::LIST;
        // }
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

fn match_app(arg_matches: &ArgMatches) -> Result<Oplapp, OplError> {
    if arg_matches.is_present("app") {
        let opt = Oplapp::from_str(
            arg_matches
                .value_of("env")
                .expect("App konnte nicht ausgelesen werden"),
        );
        let oplapp = match opt {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(-1);
            }
        };

        return Ok(oplapp);
    }
    Err(OplError::new(OplErrorKind::AppNotFoundError(String::from(
        "App muss angefuehrt werden!",
    ))))
}

fn match_list(
    arg_matches: &ArgMatches,
) -> Result<(Environment, Option<u32>, LogTyp, bool), OplError> {
    let day_offset = arg_matches.value_of("day-offset");
    let mut opt_offset = Option::None;
    if day_offset.is_some() {
        let offset = day_offset
            .unwrap()
            .parse::<u32>()
            .map_err(|err| OplError::new(OplErrorKind::ParseError(err.to_string())))?;
        opt_offset = Some(offset);
    }
    let arg_typ_opt = arg_matches.value_of("log-typ");
    println!("{:?}", arg_typ_opt);
    let mut log_typ = LogTyp::ALL;
    if arg_typ_opt.is_some() {
        log_typ = LogTyp::from_str(arg_typ_opt.unwrap())?;
        println!("{}", log_typ);
    }
    let arg_fetch = match arg_matches.is_present("fetch") {
        true => true,
        false => false,
    };
    let environment = match_env(arg_matches);

    Ok((environment, opt_offset, log_typ, arg_fetch))
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
