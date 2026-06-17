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
                let mut request = String::new();

                let mut reader = BufReader::new(&stream);
                reader.read_line(&mut request)?;

                // example: GET /index.html HTTP/1.1
                let tokens = request.splitn(3, ' ').collect::<Vec<&str>>();
                let path = tokens[1];

                let response = if path == "/" {
                    "HTTP/1.1 200 OK\r\n\r\n".to_string()
                } else if path.starts_with("/echo/") {
                    let string = path[1..].splitn(2, '/').skip(1).next().unwrap();

                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        string.len(),
                        string
                    )
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
