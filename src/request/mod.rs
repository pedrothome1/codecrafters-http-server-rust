use crate::header::Headers;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub encode_with: Option<String>,
}

impl Request {
    pub fn new(
        method: &str,
        path: &str,
        version: &str,
        headers: Headers,
        body: Vec<u8>,
        encode_with: Option<String>,
    ) -> Self {
        Self {
            method: method.to_ascii_uppercase(),
            path: path.to_owned(),
            version: version.to_owned(),
            headers,
            body,
            encode_with,
        }
    }
}
