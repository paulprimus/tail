mod config;
mod error;
mod http;
mod opltyp;
mod parse;
mod term;

extern crate clap;
extern crate crossterm;

use clap::{App, AppSettings, Arg};

use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::http::{fetch_url, HttpData};
use crate::opltyp::OplTyp;
//use crate::term::enter_alternate_screen;

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
                .subcommand(
                    App::new("serve").arg(
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

    let config: Config = config::parse().expect("Fehler beim Parsen der Konfigurationsdatei!");
    match matches.subcommand_name() {
        Some("list") => {
            print!("- {}", OplTyp::DQM);
            print!("- {}", OplTyp::FOMIS);
        }
        Some("fomis") => { /*Nichts*/ }
        _ => unreachable!(),
    };

    let mut out_locked = tokio::io::stdout();
    // let mut out_locked = out.lock();

    match matches.subcommand() {
        ("fomis", Some(fomis_matches)) => match fomis_matches.subcommand() {
            ("serve", Some(serve_matches)) => {
                let day_offset = serve_matches.value_of("day-offset");
                let mut offset: u32 = 0;
                if day_offset.is_some() {
                    offset = day_offset
                        .unwrap()
                        .parse::<u32>()
                        .map_err(|_| OplError::new(OplErrorKind::ParseError))?;
                }
                if let Some(data) = fetch_url(OplTyp::FOMIS, &config).await? {
                    print_root(&mut out_locked, data, offset).await?;
                }
            }
            ("config", Some(_config_matches)) => {
                println!("{}", config.get_config_for(OplTyp::FOMIS)?)
            }
            // ("list", None) => {}
            _ => unreachable!(),
        },
        // ("list", None) => {
        //     println!("Listtststststst");
        // }
        // _ => unreachable!(),
        _ => unreachable!(),
    }
    Ok(())
}

pub async fn print_root(
    stdout: &mut tokio::io::Stdout,
    data: HttpData,
    day_offset: u32,
) -> Result<(), OplError> {
    let ergebnis = parse::parse_root(data)?;
    term::print_root(stdout, ergebnis, day_offset).await?;
    Ok(())
}
