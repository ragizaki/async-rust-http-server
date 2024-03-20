use std::collections::HashMap;
use std::str::FromStr;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]

pub struct ParseRequestError;

impl FromStr for Request {
    type Err = ParseRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();

        // processing status line
        let status_line = iter.next().unwrap();
        let mut parts = status_line.split_whitespace();
        let method = parts.next().unwrap().to_string();
        let path = parts.next().unwrap().to_string();

        // processing headers
        let mut headers = HashMap::new();
        for header in iter {
            if header.is_empty() {
                break;
            }
            let mut header_iter = header.split(": ");
            let key = header_iter.next().unwrap().to_string();
            let val = header_iter.next().unwrap().to_string();
            headers.insert(key, val);
        }

        Ok(Request {
            method,
            path,
            headers,
        })
    }
}
