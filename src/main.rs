mod request;

use request::Request;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let request = match read_stream(&mut stream).await {
                Ok(req) => req,
                Err(msg) => return Err(msg),
            };
            parse_request(request, &mut stream).await;
            Ok(())
        });
    }
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
