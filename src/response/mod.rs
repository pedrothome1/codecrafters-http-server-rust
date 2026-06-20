use crate::header::Headers;
use std::fmt;
use std::fmt::Formatter;

pub struct Response {
    status: String,
    pub headers: Headers,
    body: Option<String>,
}

impl Response {
    pub fn new(status: &str, headers: Headers, body: Option<&str>) -> Self {
        Response { status: status.to_owned(), headers, body: body.map(|s| s.to_owned()) }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut headers = self.headers.clone();

        headers.set(
            "Content-Length",
            self.body.as_ref().map(|b| b.len().to_string()).unwrap_or_else(|| "0".to_owned()),
        );

        let mut header_lines: Vec<String> = Vec::with_capacity(headers.len() + 1);
        header_lines.push(format!("HTTP/1.1 {}", self.status));

        for header in &headers {
            header_lines.push(format!("{}: {}", header.0, header.1));
        }

        let header_str = header_lines.join("\r\n");

        let mut response = vec![&header_str, ""];
        if self.body.is_some() {
            response[1] = self.body.as_ref().unwrap();
        }
        let response = response.join("\r\n\r\n");

        write!(f, "{}", response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_without_body_to_string() {
        let mut headers = Headers::new();
        headers.add("Content-Type", "text/plain");
        headers.add("Content-Length", "20");

        let response = Response::new("200 OK", headers, None);

        assert_eq!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 20\r\n\r\n",
            response.to_string()
        );
    }

    #[test]
    fn response_with_body_to_string() {
        let body = "hello world";

        let mut headers = Headers::new();
        headers.add("Content-Type", "text/plain");

        let response = Response::new("200 OK", headers, Some(body));

        let expected = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        assert_eq!(expected, response.to_string());
    }

    #[test]
    fn content_length_gets_overriden_by_body_length_and_appears_at_the_end() {
        let body = "hello world";

        let mut headers = Headers::new();
        headers.add("Content-Length", "123");
        headers.add("Content-Type", "text/plain");

        let response = Response::new("200 OK", headers, Some(body));

        let expected = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        assert_eq!(expected, response.to_string());
    }
}
