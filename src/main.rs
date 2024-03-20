mod request;
mod response;

use request::Request;
use response::{HttpStatus, Response};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let directory = args.iter().enumerate().find_map(|(i, arg)| {
        if arg == "--directory" {
            args.get(i + 1)
        } else {
            None
        }
    });

    let directory = Arc::new(directory.map(|dir| dir.to_owned()));

    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        let directory = Arc::clone(&directory);

        tokio::spawn(async move {
            let request = match read_stream(&mut stream).await {
                Ok(req) => req,
                Err(msg) => return Err(msg),
            };
            let response = match parse_request(request, directory).await {
                Ok(res) => res,
                Err(msg) => return Err(msg),
            };
            write_stream(response, &mut stream).await
        });
    }
}

async fn read_stream(stream: &mut TcpStream) -> io::Result<Request> {
    let mut buf = [0; MAX_BUF_SIZE];
    let num_bytes = stream.read(&mut buf).await?;
    let data = std::str::from_utf8(&buf[..num_bytes]).unwrap();
    Ok(Request::from_str(data).expect("Failed to parse data"))
}

async fn write_stream(output: String, stream: &mut TcpStream) -> io::Result<()> {
    stream.write_all(output.as_bytes()).await?;
    stream.flush().await
}

async fn parse_request(
    request: Request,
    directory: Arc<Option<String>>,
) -> tokio::io::Result<String> {
    let mut iter = request.path.split('/');

    // throw away value
    iter.next();

    let response = match iter.next().unwrap() {
        "echo" => {
            let mut headers = HashMap::new();
            let body: String = iter.collect::<Vec<&str>>().join("/");
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            headers.insert("Content-Length".to_string(), body.len().to_string());

            Response::new(HttpStatus::Ok, Some(headers), Some(body))
        }
        "user-agent" => {
            let mut headers = HashMap::new();
            let user_agent = request.headers.get("User-Agent").unwrap();
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            headers.insert("Content-Length".to_string(), user_agent.len().to_string());

            Response::new(HttpStatus::Ok, Some(headers), Some(user_agent.to_owned()))
        }
        "files" => {
            if let Some(ref dir) = *directory {
                let filename = iter.next().unwrap();
                let contents = read_file(dir.to_owned(), filename).await;
                match &contents {
                    Ok(msg) => {
                        let mut headers = HashMap::new();
                        headers.insert(
                            "Content-Type".to_string(),
                            "application/octet-stream".to_string(),
                        );
                        headers.insert("Content-Length".to_string(), msg.len().to_string());

                        Response::new(HttpStatus::Ok, Some(headers), Some(msg.to_owned()))
                    }
                    Err(_) => Response::new(HttpStatus::NotFound, None, None),
                }
            } else {
                return Err(io::Error::new(io::ErrorKind::NotFound, "Problem reading"));
            }
        }
        "" => Response::new(HttpStatus::Ok, None, None),
        _ => Response::new(HttpStatus::NotFound, None, None),
    };

    Ok(response.to_string())
}

async fn read_file(directory: String, filename: &str) -> tokio::io::Result<String> {
    let file_path = format!("{directory}/{filename}");
    let file_path = Path::new(&file_path);

    if !file_path.exists() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            String::from("Couldn't find file"),
        ))
    } else {
        let mut file = File::open(file_path).await?;
        let mut buf = [0; MAX_BUF_SIZE];

        let num_bytes = file.read(&mut buf).await?;
        let contents = String::from_utf8(buf[..num_bytes].to_vec())
            .expect("Failed to convert file bytes to string");

        Ok(contents)
    }
}
