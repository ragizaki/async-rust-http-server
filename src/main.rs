use std::collections::HashMap;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    while let Ok((mut socket, _)) = listener.accept().await {
        let request = match read_stream(&mut socket).await {
            Ok(req) => req,
            Err(msg) => return Err(msg),
        };
        parse_request(request, &mut socket).await;
    }

    Ok(())
}

async fn read_stream(stream: &mut TcpStream) -> tokio::io::Result<Request> {
    let mut buf = [0; MAX_BUF_SIZE];
    let num_bytes = stream.read(&mut buf).await?;
    let data = std::str::from_utf8(&buf[..num_bytes]).unwrap();
    Ok(Request::from_str(data).expect("Failed to parse data"))
}

async fn parse_request(request: Request, stream: &mut TcpStream) {
    let mut iter = request.path.split("/");

    // throw away value
    iter.next();

    let response = match iter.next().unwrap() {
        "echo" => {
            let echoed_string: String = iter.collect::<Vec<&str>>().join("/");

            format_ok_response(echoed_string)
        }
        "user-agent" => {
            let user_agent = request.headers.get("User-Agent").unwrap();

            format_ok_response(user_agent.to_owned())
        }
        "" => format!("HTTP/1.1 200 OK\r\n\r\n"),
        _ => format!("HTTP/1.1 404 Not Found\r\n\r\n"),
    };

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn format_ok_response(body: String) -> String {
    format!(
        "{}\r\n{}\r\nContent-Length: {}\r\n\r\n{}\r\n",
        "HTTP/1.1 200 OK",
        "Content-Type: text/plain",
        body.len(),
        body
    )
}

struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRequestError;

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
        while let Some(header) = iter.next() {
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
