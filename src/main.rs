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

// async fn read_page(url: &str) -> Result<(), Box<dyn Error>> {
//     let uri = url.parse::<hyper::Uri>()?;
//
//     let mut data = HttpData::new();
//     tokio::select! {
//       Ok(Some(result)) = fetch_url(uri) => data = result,
//     _ = signal::ctrl_c() => println!("Abbruch!"),
//     _ = time::delay_for(Duration::from_secs(5)) => println!("Timeout while fetching!"),
//     };
//     let stdout_unlocked = io::stdout();
//     let mut stdout = stdout_unlocked.lock();
//     enter_alternate_screen(&mut stdout, &mut data)?;
//     loop {
//         let userinput = read_line(&mut stdout)?;
//         if userinput.trim() == "quit" {
//             break;
//         }
//         if userinput.len() > 0 {
//             let result = prepare_output(&mut data, userinput)?;
//             write_output(&mut stdout, result)?;
//         }
//     }
//     stdout.execute(LeaveAlternateScreen)?;
//     Ok(())
// }

// fn enter_alternate_screen(
//     stdout: &mut StdoutLock,
//     http_data: &mut HttpData,
// ) -> Result<(), Box<dyn Error>> {
//     stdout.queue(EnterAlternateScreen)?;
//     stdout.queue(SetForegroundColor(Color::Magenta))?;
//     stdout.queue(Print("url: "))?;
//     stdout.queue(Print(&http_data.url))?;
//     stdout.queue(Print("\n"))?;
//     stdout.queue(ResetColor)?;
//     stdout.queue(cursor::MoveDown(1))?;
//     stdout.queue(cursor::SavePosition)?;
//     let data = &http_data.body;
//     if data.len() > 10 {
//         for d in &data[..10] {
//             stdout.queue(Print(String::from_utf8(d.to_vec())?))?;
//         }
//     }
//
//     let term_size = terminal::size()?;
//     stdout.queue(cursor::MoveTo(0, term_size.1))?;
//     stdout.queue(SetForegroundColor(Color::Green))?;
//
//     stdout.queue(Print("\n"))?;
//     let length = data.len();
//     stdout.queue(Print(length))?;
//     stdout.queue(Print("\n"))?;
//     stdout.queue(Print("> ".to_string()))?;
//     stdout.queue(ResetColor)?;
//     stdout.flush()?;
//     Ok(())
// }
//
// fn write_output(stdout: &mut StdoutLock, data: Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
//     let term_size = terminal::size()?;
//     stdout.queue(Print("\n\n"))?;
//     for d in data {
//         stdout.write(&d[..])?;
//     }
//     // stdout.queue(cursor::SavePosition)?;
//     stdout.queue(cursor::MoveTo(0, term_size.1))?;
//     stdout.queue(SetForegroundColor(Color::Green))?;
//     stdout.queue(Print("> ".to_string()))?;
//     stdout.queue(ResetColor)?;
//     stdout.flush()?;
//     Ok(())
// }
//
// fn prepare_output(
//     http_data: &mut HttpData,
//     userinput: String,
// ) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
//     let body = &http_data.body;
//
//     let mut buffer = Vec::<Vec<u8>>::new();
//     for line in body {
//         let sdf = std::str::from_utf8(&line.as_slice())?;
//         if sdf.contains(userinput.as_str()) {
//             buffer.push(line.to_vec());
//         }
//     }
//     Ok(buffer)
// }
//
// fn read_line(stdout: &mut StdoutLock<'_>) -> Result<String, Box<dyn Error>> {
//     let mut line = String::new();
//     while let Event::Key(KeyEvent { code, .. }) = read()? {
//         match code {
//             KeyCode::Enter => {
//                 break;
//             }
//             KeyCode::Char(c) => {
//                 stdout.execute(Print(c))?;
//                 line.push(c);
//                 // stdout.execute(Print(&line))?;
//             }
//             KeyCode::Backspace => {
//                 let length = line.len();
//                 if length > 0 {
//                     line.truncate(length - 1);
//                     stdout.execute(cursor::MoveLeft(1))?;
//                     stdout.execute(terminal::Clear(ClearType::UntilNewLine))?;
//                 }
//             }
//             _ => {}
//         }
//     }
//
//     return Ok(line);
// }

// async fn fetch_url(url: hyper::Uri) -> Result<Option<HttpData>, Box<dyn std::error::Error>> {
//     let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
//     let client = hyper::Client::builder().build::<_, hyper::Body>(https);
//
//     let mut http_data = HttpData::new();
//     http_data.url = url.to_string();
//     let res = client.get(url).await?;
//     let status_code = res.status();
//     http_data.status = status_code.to_string();
//
//     let possible_size = res.body().size_hint().lower();
//     let mut header_map = HashMap::<String, String>::new();
//     for h in res.headers() {
//         header_map.insert(String::from(h.0.as_str()), String::from(h.0.as_str()));
//     }
//
//     let buf = hyper::body::to_bytes(res).await?;
//     let mut zeile: Vec<u8> = Vec::new();
//     let mut all: Vec<Vec<u8>> = Vec::with_capacity(possible_size as usize);
//     // const MAX_SIZE: usize = 10;
//     for b in buf {
//         zeile.push(b);
//         if b == NEW_LINE {
//             all.push(zeile);
//             zeile = Vec::new();
//         }
//     }
//
//     http_data.body = all;
//     Ok(Some(http_data))
// }

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
                if let Some(mut data) = fetch_url(OplTyp::FOMIS, config).await? {
                    enter_alternate_screen(&mut out_locked, &mut data)?;
                }
            }
            Some("config") => println!("{}", config.get_config_for(OplTyp::FOMIS)?),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
