use crate::error::{OplError, OplErrorKind};
use crate::http::HttpData;
use chrono::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct RootLogs {
    pub url: String,
    pub title: String,
    pub logs: BTreeMap<Date<Utc>, Vec<String>>,
}

impl RootLogs {
    fn new() -> RootLogs {
        RootLogs {
            url: String::new(),
            title: String::new(),
            logs: BTreeMap::<Date<Utc>, Vec<String>>::new(),
        }
    }

    fn append_log(&mut self, date: Date<Utc>, log: String) -> Result<(), OplError> {
        let mut list: Vec<String> = match self.logs.get(&date) {
            Some(v) => v.to_vec(),
            None => Vec::<String>::new(),
        };
        list.push(log);
        self.logs.insert(date, list.to_vec());
        Ok(())
    }
}

const RE_PATTERN_TITEL: &str = r"<titel>.*</titel>";

pub fn parse_root(data: HttpData) -> Result<RootLogs, OplError> {
    let mut root_logs = RootLogs::new();
    let re_titel = Regex::new(RE_PATTERN_TITEL).unwrap();
    let re_log = Regex::new(r#"(<img src="/icons/text.*)"#).unwrap();
    let re_timestamp = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) \d{2}:\d{2}").unwrap();
    let title_selector = Selector::parse("title").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    root_logs.url = data.url.clone();
    for line in &data.body {
        let str = String::from_utf8(line.to_vec())
            .map_err(|err| OplError::new(OplErrorKind::ParseError(err.to_string())))?;
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
                None => {
                    return Err(OplError::new(OplErrorKind::ParseError(
                        "Jahr konnte nicht geparst werden!".to_string(),
                    )))
                }
            };
            let monat = match caps.get(2).map(|e| {
                e.as_str()
                    .parse::<u32>()
                    .expect("Monat konnte nicht ausgelesen werden!")
            }) {
                Some(y) => y,
                None => {
                    return Err(OplError::new(OplErrorKind::ParseError(
                        "Monat konnte nicht geparst werden!".to_string(),
                    )))
                }
            };
            let day = match caps.get(3).map(|e| {
                e.as_str()
                    .parse::<u32>()
                    .expect("Tag konnte nicht ausgelesen werden!")
            }) {
                Some(y) => y,
                None => {
                    return Err(OplError::new(OplErrorKind::ParseError(
                        "Tag konnte nicht geparst werden!".to_string(),
                    )))
                }
            };
            let date = Utc.ymd(year, monat, day);
            root_logs.append_log(date, v.inner_html())?;
        }
    }
    Ok(root_logs)
}

fn parse_titel(selector: &Selector, line: &str) -> Result<String, OplError> {
    let fragment = Html::parse_fragment(line);
    let v = fragment.select(&selector).next().unwrap();
    Ok(v.inner_html())
}

#[cfg(test)]
mod tests {
    use crate::http::HttpData;
    use crate::parse;
    use std::collections::HashMap;
    use std::fs;
    use std::io::{BufRead, BufReader};

    #[test]
    fn parse_root() {
        let f = fs::File::open("tests/fomis.log").expect("Datei konnte nicht geöffnet werden!");
        let reader = BufReader::new(f);
        let mut buf: Vec<Vec<u8>> = Vec::new();
        for r in reader.lines() {
            match r {
                Ok(r) => buf.push(r.into_bytes()),
                Err(e) => {
                    println!("{}", e);
                    assert!(false);
                }
            }
        }
        let data = HttpData {
            url: "testurl".to_string(),
            status: "teststatus".to_string(),
            header: HashMap::new(),
            body: buf,
        };
        let result = parse::parse_root(data);
        assert_eq!(result.err(), None);
    }
}
