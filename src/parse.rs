use crate::error::{OplError, OplErrorKind};
use crate::http::HttpData;
use crate::opltyp::OplTyp;
use chrono::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::rc::Rc;

pub struct RootLogs {
    url: String,
    title: String,
    logs: Rc<HashMap<DateTime<Utc>, Vec<String>>>,
}

impl RootLogs {
    fn new() -> RootLogs {
        RootLogs {
            url: String::new(),
            title: String::new(),
            logs: Rc::<HashMap<DateTime<Utc>, Vec<String>>>,
        }
    }

    fn append_log(&self, date: DateTime<Utc>, log: String) -> Result<(), OplError>{
        let logs:HashMap::<DateTime<Utc>, Rc<Vec<String>>> = self.logs;
        let mut list = match logs.get(&date) {
            Some(v) => v,
            None => return Err(OplError::new(OplErrorKind::ParseError)),
        };
        list.push(log);
        Ok(())
    }
}

const re_pattern_titel: &'static str = r"<titel>.*</titel>";

pub fn parse_root(data: &mut HttpData) -> Result<RootLogs, OplError> {
    let utc: DateTime<Utc> = Utc::now();
    //  println!("{}
    let mut root_logs = RootLogs::new();
    let mut logs: HashMap<DateTime<Utc>, Vec<String>> =
        HashMap::<DateTime<Utc>, Vec<String>>::new();

    let re_titel = Regex::new(re_pattern_titel).unwrap();
    let re_log = Regex::new(r#"(<img src="/icons/text.*)"#).unwrap();
    let re_timestamp = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) \d{2}:\d{2}").unwrap();
    let title_selector = Selector::parse("title").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    root_logs.url = data.url.clone();
    for line in &data.body {
        let str = String::from_utf8(line.to_vec())
            .map_err(|_| OplError::new(OplErrorKind::ParseError))?;
        if re_titel.is_match(str.as_str()) {
            root_logs.title = parse_titel(&title_selector, &str)?;
        } else if re_log.is_match(str.as_str()) {
            let fragment = Html::parse_fragment(&str);
            let v = fragment.select(&a_selector).next().unwrap();
            let caps = re_timestamp
                .captures(&str)
                .expect("Zeitstempel konnte nicht ausgelesen werden!");
            let year = match caps.get(1).map(|e| {
                e.as_str()
                    .parse::<i32>()
                    .expect("Jahr konnte nicht ausgelesen werden!")
            }) {
                Some(y) => y,
                None => return Err(OplError::new(OplErrorKind::ParseError)),
            };
            let monat = match caps.get(2).map(|e| {
                e.as_str()
                    .parse::<u32>()
                    .expect("Monat konnte nicht ausgelesen werden!")
            }) {
                Some(y) => y,
                None => return Err(OplError::new(OplErrorKind::ParseError)),
            };
            let day = match caps.get(3).map(|e| {
                e.as_str()
                    .parse::<u32>()
                    .expect("Tag konnte nicht ausgelesen werden!")
            }) {
                Some(y) => y,
                None => return Err(OplError::new(OplErrorKind::ParseError)),
            };
            let date = Utc.ymd(year, monat, day);
            // root_logs.
            // lines.push(v.inner_html().into_bytes());
            // lines.push(b" ".to_vec());
            // lines.push(timestamp.as_bytes().to_vec());
            // lines.push(b"\n".to_vec());
        }
    }
    Ok(root_logs)
}

fn parse_titel(selector: &Selector, line: &str) -> Result<String, OplError> {
    let fragment = Html::parse_fragment(line);
    let v = fragment.select(&selector).next().unwrap();
    Ok(v.inner_html())
}
