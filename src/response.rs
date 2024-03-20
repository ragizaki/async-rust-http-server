// use std::collections::HashMap;

// enum HttpStatus {
//     Ok,
//     NotFound,
// }

// pub struct Response {
//     status: HttpStatus,
//     headers: HashMap<String, String>,
//     body: String,
// }

// impl Response {
//     fn format_status_line(&self) -> String {
//         match self.status {
//             HttpStatus::Ok => String::from("HTTP/1.1 200 OK\r\n\r\n"),
//             HttpStatus::NotFound => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
//         }
//     }

//     fn format_headers(&self) -> String {
//         let mut result = String::new();

//         for (key, value) in self {
//             result.push_str(&format!("{}: {}\n", key, value));
//         }
//         result
//     }
// }

// impl ToString for Response {
//     fn to_string(&self) -> String {
//         let result = String::new();

//         result.push_str(&self.format_status_line());
//         result.push_str(&self.format_headers());
//     }
// }
