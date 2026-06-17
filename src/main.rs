use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:4221";
    let listener = TcpListener::bind(addr)?;

    eprintln!("Running server at {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                let mut user_agent = String::new();

                reader.read_line(&mut request_line)?;

                loop {
                    let mut header = String::new();
                    reader.read_line(&mut header)?;

                    if header == "\r\n" {
                        break;
                    }

                    let mut split = header.splitn(2, ':');
                    let (name, value) = (split.next().unwrap(), split.next().unwrap());

                    if name.eq_ignore_ascii_case("User-Agent") {
                        user_agent = value.trim().to_string();
                    }
                }

                // example: GET /index.html HTTP/1.1
                let tokens = request_line.splitn(3, ' ').collect::<Vec<&str>>();
                let path = tokens[1];

                let response = if path == "/" {
                    "HTTP/1.1 200 OK\r\n\r\n".to_string()
                } else if path.starts_with("/echo/") {
                    let string = path[1..].splitn(2, '/').skip(1).next().unwrap();
                    build_content_response("200 OK", "text/plain", string)
                } else if path == "/user-agent" {
                    build_content_response("200 OK", "text/plain", &user_agent)
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                };

                stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn build_content_response(status: &str, content_type: &str, content: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content_type,
        content.len(),
        content
    )
}
