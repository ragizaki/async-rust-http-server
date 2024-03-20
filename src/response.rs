use std::collections::HashMap;

pub enum HttpStatus {
    Ok,
    NotFound,
    Created,
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => String::from("200 OK"),
            Self::NotFound => String::from("404 Not Found"),
            Self::Created => String::from("201 Created"),
        }
    }
}

pub struct Response {
    status: HttpStatus,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl Response {
    pub fn new(
        status: HttpStatus,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status.to_string());

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                response.push_str(&format!("{}: {}\r\n", key, value));
            }
        }

        response.push_str("\r\n");

        if let Some(body) = &self.body {
            response.push_str(body);
        }

        response
    }
}
