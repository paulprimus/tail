use crate::error::{OplError, OplErrorKind};
use crate::http::HttpData;
use crate::logtyp::LogTyp;
use crate::oplcmd::OplCmd;
use crate::opldate::OplDate;
use chrono::prelude::*;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct RootLogs {
    pub url: String,
    pub title: String,
    pub logs: Vec<RootLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootLog {
    pub log_typ: LogTyp,
    pub name: String,
    pub date: OplDate,
}

impl RootLogs {
    fn new() -> RootLogs {
        RootLogs {
            url: String::new(),
            title: String::new(),
            logs: Vec::<RootLog>::new(),
        }
    }

    fn add_log(&mut self, root_log: RootLog) -> Result<(), OplError> {
        self.logs.push(root_log);
        Ok(())
    }

    pub fn get_logs_by_date(
        &self,
        logtyp: &LogTyp,
    ) -> Result<BTreeMap<OplDate, Vec<RootLog>>, OplError> {
        let mut map: BTreeMap<OplDate, Vec<RootLog>> = BTreeMap::new();
        for l in &self.logs {
            let mut list: Vec<RootLog> = match map.get(&l.date) {
                Some(v) => v.to_vec(),
                None => Vec::<RootLog>::new(),
            };
            if *logtyp == LogTyp::ALL {
                let v = l.date.to_owned();
                list.push(l.clone());
                map.insert(v, list);
            } else {
                if l.log_typ == *logtyp {
                    list.push(l.to_owned());
                }
                if !list.is_empty() {
                    let v = l.date.to_owned();
                    map.insert(v, list);
                }
            }
        }
        Ok(map)
    }
}

const RE_PATTERN_TITEL: &str = r"<titel>.*</titel>";
const RE_PATTERN_ACCESS_LOG: &str = r"access.*.log";
const RE_PATTERN_START_LOG: &str = r"start.*.log";

pub fn parse_root(data: HttpData) -> Result<RootLogs, OplError> {
    let mut root_logs = RootLogs::new();
    let re_titel = Regex::new(RE_PATTERN_TITEL).unwrap();
    let re_log = Regex::new(r#"(<img src="/icons/text.*)"#).unwrap();
    let re_access_log = Regex::new(RE_PATTERN_ACCESS_LOG).unwrap();
    let re_start_log = Regex::new(RE_PATTERN_START_LOG).unwrap();
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
            let log_file_name = v.inner_html();
            let mut log_typ = LogTyp::ALL;
            if re_access_log.is_match(&log_file_name) {
                log_typ = LogTyp::ACCESS;
            } else if re_start_log.is_match(&log_file_name) {
                log_typ = LogTyp::START;
            } else {
                log_typ = LogTyp::LOG;
            }
            let root_log = RootLog {
                name: log_file_name,
                log_typ: log_typ,
                date: OplDate::from(date),
            };
            root_logs.add_log(root_log)?;
        }
    }
    Ok(root_logs)
}

fn parse_titel(selector: &Selector, line: &str) -> Result<String, OplError> {
    let fragment = Html::parse_fragment(line);
    let v = fragment.select(&selector).next().unwrap();
    Ok(v.inner_html())
}

const ROOTLOGS_JSON_FILE_NAME_TEMPLATE: &str = "rootlogs{}.json";
const OBJECTS_PATH: &str = "objects";
pub async fn write_json(root_logs: &mut RootLogs, oplcmd: &OplCmd) -> Result<(), OplError> {
    let root_logs_string = serde_json::to_string_pretty(root_logs)?;

    //let v = serde_json::from_str(&root_logs_string)?;
    let objects_dir = Path::new(OBJECTS_PATH);
    if !objects_dir.exists() {
        tokio::fs::create_dir(objects_dir).await?;
    }

    let filename = get_filename(oplcmd).expect("Dateiname konnte nicht zusammengesetzt werden!");
    let json_file_path = objects_dir.join(Path::new(&filename));
    let mut json_file: File;
    if json_file_path.exists() {
        json_file = File::open(json_file_path).await?;
    } else {
        json_file = File::create(json_file_path).await?;
    }
    json_file.write_all(root_logs_string.as_bytes()).await?;
    json_file.flush().await?;
    Ok(())
}

fn get_filename(oplcmd: &OplCmd) -> Result<String, OplError> {
    let mut filename: String = String::from("error.json");

    let result = match oplcmd {
        // OplCmd::FOMIS(_) => {
        //     filename = ROOTLOGS_JSON_FILE_NAME_TEMPLATE.replace("{}", "_fomis");
        //     Ok(filename)
        // }
        // OplCmd::DQM(_) => {
        //     filename = ROOTLOGS_JSON_FILE_NAME_TEMPLATE.replace("{}", "_dqm");
        //     Ok(filename)
        // }
        OplCmd::CONFIG => Err(OplError::new(OplErrorKind::RootLogError)),
        OplCmd::LIST(listcmd) => Err(OplError::new(OplErrorKind::RootLogError)),
        OplCmd::VIEW => Err(OplError::new(OplErrorKind::RootLogError)),
    };
    result
}

pub async fn read_local_rootlogs(oplcmd: &OplCmd) -> Result<RootLogs, OplError> {
    let objects_dir = Path::new(OBJECTS_PATH);
    let filename = get_filename(oplcmd).expect("Dateiname konnte nicht zusammengesetzt werden!");
    let json_file_path = objects_dir.join(Path::new(&filename));
    if !json_file_path.exists() {
        return Err(OplError::new(OplErrorKind::FileNotFound(String::from(
            "Datei zuerst laden!",
        ))));
    }
    let s = tokio::fs::read_to_string(json_file_path).await?;
    let rootlogs: RootLogs = serde_json::from_str(&s)?;
    Ok(rootlogs)
}

#[cfg(test)]
mod tests {
    use crate::http::HttpData;
    use crate::rootlogs;
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
        let result = rootlogs::parse_root(data);
        assert_eq!(result.err(), None);
    }
}
