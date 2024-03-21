use std::collections::HashMap;
use std::str::FromStr;

pub enum HttpMethod {
    Get,
    Post,
}

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
        let mut parts = s.split("\r\n\r\n");
        let mut header_lines = parts.next().ok_or(ParseRequestError)?.lines();
        let body = parts.next();

        let mut status_line_parts = header_lines
            .next()
            .ok_or(ParseRequestError)?
            .split_whitespace();

        let method_str = status_line_parts.next().ok_or(ParseRequestError)?;
        let method = match method_str {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => return Err(ParseRequestError),
        };

        let path = status_line_parts
            .next()
            .ok_or(ParseRequestError)?
            .to_string();

        let headers = header_lines
            .take_while(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let mut parts = line.split(": ");
                let key = parts.next()?.to_string();
                let value = parts.next()?.to_string();
                Some((key, value))
            })
            .collect();

        Ok(Request {
            method,
            path,
            headers,
            body: body.map(String::from),
        })
    }
}
