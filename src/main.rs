extern crate clap;

use std::collections::VecDeque;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::io;

use clap::{App, Arg};
use std::fs::File;

#[derive(Debug)]
struct LinesWithEnding<B> {
    buf: B,
}

impl<B: BufRead> Iterator for LinesWithEnding<B> {
    type Item = std::io::Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

fn lines_with_ending<B: BufRead>(reader: B) -> LinesWithEnding<B> {
    LinesWithEnding { buf: reader }
}

fn tail<File, W: Write>(input: File, output: W, num: usize) -> io::Result<()> {
    println!("{}", num);
    let lines = lines_with_ending(BufReader::new(input));
    let mut writer = io::BufWriter::new(output);

    let mut deque = VecDeque::new();
    for lr in lines {
        match lr {
            Ok(l) => deque.push_back(l),
            Err(err) => return Err(err),
        }
    }
    for line in deque {
        writer.write(line.as_bytes());
    }
    Ok(())
}

//fn

fn main() {
    let matches = App::new("tail - following logs made easy!")
        .version("0.0.1")
        .author("Paul Pacher")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .help("Datei anführen.")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    if let Some(filename) = matches.value_of("file") {
         let file = match File::open(filename) {
             Ok(f) => f,
             Err(e ) => {
                 println!("Fehler: Datei konnte nicht geöffnet werden!");
                 return
             }

         };


        let stdout = io::stdout();
        let mut stdoutLock = stdout.lock();
        tail(file,stdoutLock, 10);
    }



    // let stdout = io::stdout();
    // let mut stdout_lock = stdout.lock();
    // let stdin = io::stdin();
    // let mut stdin_lock = stdin.lock();
    // let x: Vec<String> = std::env::args().skip(1).collect();
    // for s in x {
    //     println!("{}", s);
    // }
   // tail(&mut stdin_lock, &mut stdout_lock, 10);
}
