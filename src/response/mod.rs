use crate::header::Headers;

pub struct Response {
    status: String,
    pub headers: Headers,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: impl Into<String>, headers: Headers, body: Option<Vec<u8>>) -> Self {
        Response { status: status.into(), headers, body }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut headers = self.headers.clone();
        let content_length = self.body.as_ref().map(|b| b.len()).unwrap_or_else(|| 0);
        headers.set("Content-Length", content_length.to_string());

        let mut header_lines: Vec<String> = Vec::with_capacity(headers.len() + 2);
        header_lines.push(format!("HTTP/1.1 {}", self.status));

        for header in &headers {
            header_lines.push(format!("{}: {}", header.0, header.1));
        }

        let full_header = header_lines.join("\r\n");

        let mut response: Vec<u8> = Vec::with_capacity(full_header.len() + content_length + 4);
        response.extend_from_slice(full_header.as_bytes());
        response.extend_from_slice(b"\r\n\r\n");

        if let Some(body) = self.body.as_ref() {
            response.extend_from_slice(body)
        }

        response
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
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n",
            String::from_utf8(response.as_bytes()).unwrap()
        );
    }

    #[test]
    fn response_with_body_to_string() {
        let body = "hello world";

        let mut headers = Headers::new();
        headers.add("Content-Type", "text/plain");

        let response = Response::new("200 OK", headers, Some(body.as_bytes().to_vec()));

        let expected = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        assert_eq!(expected, String::from_utf8(response.as_bytes()).unwrap());
    }

    #[test]
    fn content_length_gets_overriden_by_body_length_and_appears_at_the_end() {
        let body = "hello world";

        let mut headers = Headers::new();
        headers.add("Content-Length", "123");
        headers.add("Content-Type", "text/plain");

        let response = Response::new("200 OK", headers, Some(body.as_bytes().to_vec()));

        let expected = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        assert_eq!(expected, String::from_utf8(response.as_bytes()).unwrap());
    }
}
