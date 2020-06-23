extern crate clap;

use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, StdoutLock, Write};

use std::fmt;
use std::fs::File;
use std::time::Duration;

use clap::{App, Arg};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio::io::{self as tio, AsyncBufReadExt, AsyncWriteExt};
use tokio::signal;
use tokio::time::{self};

const NEW_LINE: u8 = b'\n';
const SAVE_CURSOR_POSITION: [u8; 3] = [27, 91, 115];
const RESTORE_CURSOR_POSITION: [u8; 3] = [27, 91, 117];

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

fn tail<R: Read>(input: R, num: usize) -> io::Result<()> {
    println!("tail!!!");
    let stdout_lock = io::stdout();
    //let mut reader = BufReader::new(input);
    let mut writer = io::BufWriter::new(stdout_lock);
    let lines = lines_with_ending(io::BufReader::new(input)).skip(num);

    let mut deque = VecDeque::new();
    for line in lines {
        match line {
            Ok(l) => {
                deque.push_back(l);
                if deque.len() > num {
                    deque.pop_front();
                }
            }
            Err(err) => return Err(err),
        }
    }
    for line in deque {
        writer.write(line.as_bytes())?;
    }
    Ok(())
}

#[tokio::main]
async fn read_page(url: &str) -> Result<(), Box<dyn Error>> {
    println!("read page");
    println!("{}", url);
    let uri = url.parse::<hyper::Uri>()?;

    let mut data: VecDeque<Vec<u8>> = VecDeque::new();
    tokio::select! {
      Ok(Some(result)) = fetch_url(uri) => data = result,
    _ = signal::ctrl_c() => println!("Abbruch!"),
    _ = time::delay_for(Duration::from_secs(5)) => println!("Timeout while fetching!"),
    };

    // let stdout_unlocked = io::stdout();
    // let mut stdout = stdout_unlocked.lock();
    let mut stdout = tio::stdout();
    stdout.write(&SAVE_CURSOR_POSITION).await?;
    //println!("{}","b\x1b[s");
    //stdout.write(b"\x1b[s")?;
    loop {
        let userinput = match read_user_input().await {
            Ok(v) => match v {
                Some(d) => d,
                None => {
                    println!("break");
                    break;
                }
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        };
        if std::str::from_utf8(&userinput[..])?.trim() == "quit" {
            break;
        }
        //println!("{}", "b\x1b[u");
        for d in &data {
            stdout.write(&d[..]).await?;
        }
        stdout.write(&userinput[..]).await?;
        stdout.write(&RESTORE_CURSOR_POSITION).await?;
        //stdout.write(b"\x1b[u")?;
    }
    Ok(())
}

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

//impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

async fn read_user_input() -> Result<Option<Vec<u8>>, MyError> {
    let stdin = tio::stdin();
    let mut reader = tokio::io::BufReader::new(stdin);
    let mut buffer: Vec<u8> = Vec::new();
    tokio::select! {
        _ = reader.read_until(b'\n', &mut buffer) => println!("finished reading stdin"),
        _ = time::delay_for(Duration::from_secs(10)) =>  return Ok(None)
    };
    // match reader.read_until(b'\n', &mut buffer).await {
    //     Ok(_v) => (),
    //     Err(_e) => return Err(MyError::new("Fehler"))
    // }
    return Ok(Some(buffer));
}

// fn read_user_input() -> Result<Option<Vec<u8>>, MyError> {
//     let stdin = io::stdin();
//     let stdin_lock = stdin.lock();
//     let mut buffer: String = String::new();
//     let mut reader = std::io::BufReader::new(stdin_lock);
//
//     match reader.read_line( &mut buffer) {
//         Ok(v) => println!("done reading {}", v),
//         Err(_e) => return Err(MyError::new("Fehler"))
//     }
//     return Ok(Some(buffer.into_bytes()));
// }

async fn fetch_url(
    url: hyper::Uri,
) -> Result<Option<VecDeque<Vec<u8>>>, Box<dyn std::error::Error>> {
    println!("fetch_url");
    let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(url).await?;

    println!("Status: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let buf = hyper::body::to_bytes(res).await?;
    let mut zeile: Vec<u8> = Vec::new();
    let mut all: VecDeque<Vec<u8>> = VecDeque::with_capacity(11);
    const MAX_SIZE: usize = 10;
    for b in buf {
        zeile.push(b);
        if b == NEW_LINE {
            all.push_back(zeile);
            if all.len() > MAX_SIZE {
                all.pop_front();
            }
            zeile = Vec::new();
        }
    }
    println!("\n\nDone!");
    Ok(Some(all))
}

fn follow(filename: &str, _num: usize) -> io::Result<()> {
    //println!("{}", num);
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let file = File::open(filename)?;
    let mut writer = io::BufWriter::new(stdout_lock);
    let mut buf: Vec<u8> = Vec::new();

    let mut reader = BufReader::new(file);
    let mut cur_seek_pos: u64 = reader.seek(SeekFrom::End(0))?;
    let mut last_seek_pos: u64 = cur_seek_pos;
    loop {
        std::thread::sleep(Duration::from_secs(3));
        cur_seek_pos = reader.seek(SeekFrom::End(0))?;
        if cur_seek_pos > last_seek_pos {
            reader.seek(SeekFrom::Start(last_seek_pos))?;
        } else {
            reader.seek(SeekFrom::Start(0))?;
        }
        buf.clear();
        reader.read_to_end(&mut buf)?;
        writer.write_all(&buf[..])?;
        writer.flush()?;
        last_seek_pos = cur_seek_pos;
    }
}

fn main() {
    let matches = App::new("tail - following logs made easy!")
        .version("0.0.1")
        .author("Paul Pacher")
        .arg(
            Arg::with_name("print")
                .long("print")
                .short("p")
                .help("Datei anführen.")
                .value_name("FILE")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("follow")
                .long("follow")
                .short("f")
                .value_name("FILE")
                .help("Schreibt Änderungen in die Standardausgabe.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("http")
                .long("http")
                .short("s")
                .value_name("URL")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    if let Some(filename) = matches.value_of("print") {
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("Fehler: Datei konnte nicht geöffnet werden!\n {}", e);
                return;
            }
        };
        match tail(file, 10) {
            Ok(()) => println!("Success"),
            Err(e) => println!("{}", e),
        }
    } else if let Some(filename) = matches.value_of("follow") {
        match follow(filename, 10) {
            Ok(()) => println!("Success"),
            Err(e) => println!("{}", e),
        }
    } else if let Some(url) = matches.value_of("http") {
        match read_page(url) {
            Ok(()) => println!("Success"),
            Err(e) => println!("{}", e),
            //_ => {}
        };
    } else {
        println!("no match!");
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
