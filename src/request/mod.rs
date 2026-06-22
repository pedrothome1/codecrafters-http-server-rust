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
        method: impl Into<String>,
        path: impl Into<String>,
        version: impl Into<String>,
        headers: Headers,
        body: Vec<u8>,
        encode_with: Option<String>,
    ) -> Self {
        let mut method = method.into();
        method.make_ascii_uppercase();

        Self {
            method,
            path: path.into(),
            version: version.into(),
            headers,
            body,
            encode_with,
        }
    }
}
