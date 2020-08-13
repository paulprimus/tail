use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use hyper::{body::HttpBody, client::HttpConnector};
use hyper_tls::HttpsConnector;

use crate::config::Config;
use crate::error::{OplError, OplErrorKind};
use crate::opltyp::OplTyp;

const NEW_LINE: u8 = b'\n';

#[derive(Debug, Clone)]
pub struct HttpData {
    pub url: String,
    pub status: String,
    pub header: HashMap<String, String>,
    pub body: Vec<Vec<u8>>,
}

impl HttpData {
    pub fn new() -> HttpData {
        HttpData {
            url: "".to_string(),
            status: "".to_string(),
            header: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl fmt::Display for HttpData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "URL: {}", self.url)?;
        if self.status != "".to_string() {
            writeln!(f, "Status: {}", self.status)?;
        }
        for h in &self.header {
            writeln!(f, "{}={}", h.0, h.1)?;
        }
        Ok(())
    }
}

//pub async fn fetch_url(url: hyper::Uri) -> Result<Option<HttpData>, OplError> {
pub async fn fetch_url(opltyp: OplTyp, config: Config) -> Result<Option<HttpData>, OplError> {
    let url = config.get_url_for(opltyp)?;
    let hyper_uri = url.parse::<hyper::Uri>()?;
    let https: HttpsConnector<HttpConnector> = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let mut http_data = HttpData::new();
    http_data.url = url.to_string();
    let res = client.get(hyper_uri).await?;
    let status_code = res.status();
    http_data.status = status_code.to_string();

    let possible_size = res.body().size_hint().lower();
    let mut header_map = HashMap::<String, String>::new();
    for h in res.headers() {
        header_map.insert(h.0.to_string(), String::from(h.1.to_str().unwrap()));
    }
    http_data.header = header_map;

    let buf = hyper::body::to_bytes(res)
        .await
        .map_err(|_| OplError::new(OplErrorKind::FileNotFound))?;
    let mut zeile: Vec<u8> = Vec::new();
    let mut all: Vec<Vec<u8>> = Vec::with_capacity(possible_size as usize);
    // const MAX_SIZE: usize = 10;
    for b in buf {
        zeile.push(b);
        if b == NEW_LINE {
            all.push(zeile);
            zeile = Vec::new();
        }
    }

    http_data.body = all;
    Ok(Some(http_data))
}

#[cfg(test)]
mod tests {
    use crate::http::HttpData;

    #[test]
    fn display() {
        let mut http_data = HttpData::new();
        http_data.url = "www.test.at".to_string();
        assert_eq!(http_data.url, "www.test.at".to_string());
        print!("{}", http_data);
        assert_eq!(format!("{}", http_data), "URL: www.test.at");
    }
}
