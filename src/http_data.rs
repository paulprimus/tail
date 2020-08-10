use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

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
        // if self.header != "".to_string() {
        //     writeln!(f, "Header: {}", self.header)?;
        // }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::http_data::HttpData;

    #[test]
    fn display() {
        let mut http_data = HttpData::new();
        http_data.url = "www.test.at".to_string();
        assert_eq!(http_data.url, "www.test.at".to_string());
        print!("{}", http_data);
        assert_eq!(format!("{}", http_data), "URL: www.test.at");
    }
}
