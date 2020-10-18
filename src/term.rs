use std::io::{StdoutLock, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    style::Print,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp},
    ExecutableCommand, QueueableCommand,
};

use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::http::{fetch_url, HttpData};
use crate::opltyp::OplTyp;
use crate::parse::RootLogs;

pub fn print_root(
    stdout: &mut StdoutLock,
    data: RootLogs,
    opl_typ: OplTyp,
) -> Result<(), OplError> {
    ScrollUp(20);
    // stdout.queue(SetForegroundColor(Color::Magenta))?;

    //stdout.queue(SetForegroundColor(Color::Magenta))?;
    //stdout.queue( Print("url: "))?;
    // stdout.queue(Print(&h))
    // println!("{}", Print(&http_data.url));
    stdout.queue(Print("\n"))?;

    // let data = data;
    // for d in &data[..] {
    //     stdout.queue(Print(
    //         String::from_utf8(d.to_vec()).map_err(|_| OplError::new(OplErrorKind::Utf8Error))?,
    //     ))?;
    // }
    stdout.flush()?;
    Ok(())
}

pub fn enter_alternate_screen(
    stdout: &mut StdoutLock,
    http_data: &mut HttpData,
) -> Result<(), OplError> {
    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(SetForegroundColor(Color::Magenta))?;
    stdout.queue(Print("url: "))?;
    stdout.queue(Print(&http_data.url))?;
    stdout.queue(Print("\n"))?;
    stdout.queue(ResetColor)?;
    stdout.queue(cursor::MoveDown(1))?;
    stdout.queue(cursor::SavePosition)?;
    let data = &http_data.body;
    //if data.len() > 10 {
    for d in &data[..] {
        stdout.queue(Print(
            String::from_utf8(d.to_vec()).map_err(|_| OplError::new(OplErrorKind::Utf8Error))?,
        ))?;
    }
    //}

    let term_size = terminal::size()?;
    stdout.queue(cursor::MoveTo(0, term_size.1))?;
    stdout.queue(SetForegroundColor(Color::Green))?;

    stdout.queue(Print("\n"))?;
    let length = data.len();
    stdout.queue(Print(length))?;
    stdout.queue(Print("\n"))?;
    stdout.queue(Print("> ".to_string()))?;
    stdout.queue(ResetColor)?;
    stdout.flush()?;
    Ok(())
}

fn write_output(stdout: &mut StdoutLock, data: Vec<Vec<u8>>) -> Result<(), OplError> {
    let term_size = terminal::size()?;
    stdout.queue(Print("\n\n"))?;
    for d in data {
        stdout.write(&d[..])?;
    }
    // stdout.queue(cursor::SavePosition)?;
    stdout.queue(cursor::MoveTo(0, term_size.1))?;
    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.queue(Print("> ".to_string()))?;
    stdout.queue(ResetColor)?;
    stdout.flush()?;
    Ok(())
}

fn prepare_output(http_data: &mut HttpData, userinput: String) -> Result<Vec<Vec<u8>>, OplError> {
    let body = &http_data.body;

    let mut buffer = Vec::<Vec<u8>>::new();
    for line in body {
        let sdf = std::str::from_utf8(&line.as_slice())
            .map_err(|_| OplError::new(OplErrorKind::Utf8Error))?;
        if sdf.contains(userinput.as_str()) {
            buffer.push(line.to_vec());
        }
    }
    Ok(buffer)
}

fn read_line(stdout: &mut StdoutLock<'_>) -> Result<String, OplError> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = read()? {
        match code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                stdout.execute(Print(c))?;
                line.push(c);
                // stdout.execute(Print(&line))?;
            }
            KeyCode::Backspace => {
                let length = line.len();
                if length > 0 {
                    line.truncate(length - 1);
                    stdout.execute(cursor::MoveLeft(1))?;
                    stdout.execute(terminal::Clear(ClearType::UntilNewLine))?;
                }
            }
            _ => {}
        }
    }

    return Ok(line);
}
