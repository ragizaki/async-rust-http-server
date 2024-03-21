use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRequestError;

impl FromStr for Request {
    type Err = ParseRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split("\r\n\r\n").collect::<Vec<_>>();
        let mut iter = chunks[0].lines();

        // processing status line
        let mut status_line = iter.next().unwrap().split_whitespace();
        let method = match status_line.next().unwrap() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => return Err(ParseRequestError),
        };
        let path = status_line.next().unwrap().to_string();

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

        match method {
            HttpMethod::Get => Ok(Request {
                method,
                path,
                headers,
                body: None,
            }),
            HttpMethod::Post => {
                let body = chunks[1];
                Ok(Request {
                    method,
                    path,
                    headers,
                    body: Some(body.to_owned()),
                })
            }
        }
    }
}
