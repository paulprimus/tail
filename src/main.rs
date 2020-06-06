extern crate clap;

use std::collections::VecDeque;
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write, Bytes};

use clap::{App, Arg};
use std::fs::File;
use std::time::Duration;

use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::Body;
use hyper_tls::HttpsConnector;
use tokio::io::AsyncWriteExt as _;
use tokio::signal;

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
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
async fn read_page(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("read page");
    println!("{}", url);
    let uri = url.parse::<hyper::Uri>()?;
    // if uri.scheme_str() != Some("https") {
    //     println!("This example only works with 'https' URLs.");
    //     return Ok(());
    // }
    //fetch_url(uri).await;
    tokio::select! {
    _ = fetch_url(uri) => println!("url fetched!"),
    _ = signal::ctrl_c() => println!("Abbruch")
    }
    Ok(())
}

async fn fetch_url(url: hyper::Uri) -> Result<(), Box<dyn std::error::Error>> {
    println!("fetch_url");
    let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(url).await?;

    println!("Status: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    println!("Body:\n");

    let http_body: &mut Body = res.body_mut();
    loop {
        if http_body.is_end_stream() {
            println!("Ende Http");
            tokio::time::delay_for(Duration::from_secs(3)).await;
            continue;
        }
        let data = http_body.data().await;
        //http_body.poll_data()

        match data {
            Some(b) => match b {
                Ok(v) => tokio::io::stdout().write_all(&v).await?,
                Err(e) => return Err(Box::new(e)),
            },
            None => tokio::time::delay_for(Duration::from_secs(3)).await,
        }
    }
    // while let Some(result) = http_body.by.await {
    //     match result {
    //         Ok(chunk) => {
    //             for b in chunk.iter() {
    //                 if
    //             }
    //         },
    //         Err(e) => return Err(Box::new(e)),
    //     }
    //     //tokio::io::stdout().write_all(&chunk?).await?;
    // }
   //  Body::channel();
   // let buf= hyper::body::to_bytes(res).await?;


    println!("\n\nDone!");
    Ok(())
}

struct MyBody;
impl  MyBody {

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
    //Ok(())
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
