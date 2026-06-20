use codecrafters_http_server::header::Headers;
use codecrafters_http_server::request::Request;
use codecrafters_http_server::response::Response;
use codecrafters_http_server::thread_pool::ThreadPool;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:4221";
    let listener = TcpListener::bind(addr)?;
    let pool = ThreadPool::new(4);

    eprintln!("Running server at {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(move || {
                if let Err(e) = handle_connection(&stream) {
                    eprintln!("Connection closed: {}", e);
                }
            }),
            Err(e) => {
                eprintln!("Error getting connection: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut reader = BufReader::new(stream);

    loop {
        let request = read_request(&mut reader)?;

        let response = if request.path == "/" {
            root_handler(&request)?
        } else if request.path.starts_with("/echo/") {
            echo_handler(&request)?
        } else if request.path == "/user-agent" {
            user_agent_handler(&request)?
        } else if request.path == "/headers" {
            print_headers_handler(&request)?
        } else {
            Response::new("404 Not Found", Headers::new(), None)
        };

        reader.get_mut().write_all(response.to_string().as_bytes())?;

        if request.headers.get("Connection").is_some_and(|h| h == "close") {
            break;
        }
    }

    Ok(())
}

type ResponseResult = Result<Response, Box<dyn Error>>;

fn root_handler(_: &Request) -> ResponseResult {
    Ok(Response::new("200 OK", Headers::new(), None))
}

fn echo_handler(request: &Request) -> ResponseResult {
    let string = request.path[1..].splitn(2, '/').skip(1).next().unwrap();

    let mut headers = Headers::new();
    headers.add("Content-Type", "text/plain");

    Ok(Response::new("200 OK", headers, Some(string)))
}

fn user_agent_handler(request: &Request) -> ResponseResult {
    let mut headers = Headers::new();
    headers.add("Content-Type", "text/plain");

    Ok(Response::new("200 OK", headers, request.headers.get("User-Agent")))
}

fn print_headers_handler(request: &Request) -> ResponseResult {
    println!("{} {} {}", request.method, request.path, request.version);
    for header in &request.headers {
        println!("{}: {}", header.0, header.1);
    }

    Ok(Response::new("200 OK", Headers::new(), None))
}

fn read_request(reader: &mut BufReader<&TcpStream>) -> Result<Request, Box<dyn Error>> {
    let mut request_line = String::new();
    let mut headers = Headers::new();

    let n = reader.read_line(&mut request_line)?;
    if n == 0 {
        return Err("reached EOF".into());
    }

    loop {
        let mut header_str = String::new();
        reader.read_line(&mut header_str)?;

        if header_str == "\r\n" {
            break;
        }

        let mut split = header_str.splitn(2, ':');
        let (name, value) = (
            split.next().ok_or("expected header key")?,
            split.next().ok_or("expected header value")?,
        );
        headers.add(name, value.trim());
    }

    let mut request_line = request_line.splitn(3, ' ');
    let (method, path, version) = (
        request_line.next().ok_or("expected request method")?,
        request_line.next().ok_or("expected request path")?,
        request_line.next().ok_or("expected http version")?.trim(),
    );

    Ok(Request::new(method, path, version, headers))
}
