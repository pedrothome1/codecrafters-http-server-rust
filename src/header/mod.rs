#![allow(unused)]

use std::iter;
use std::slice::Iter;

type Header = (String, String);

#[derive(Clone)]
pub struct Headers {
    inner: Vec<Header>,
}

impl Headers {
    pub fn new() -> Self {
        Headers { inner: Vec::new() }
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        let canon_key = canonical_header_key(key);
        self.inner.iter().find(|h| h.0 == canon_key).map(|h| &h.1[..])
    }

    pub fn values(&self, key: impl AsRef<str>) -> Vec<&str> {
        let canon_key = canonical_header_key(key);
        self.inner.iter().filter(|h| h.0 == canon_key).map(|h| &h.1[..]).collect()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn add(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        self.add_owned(canonical_header_key(key), value)
    }

    pub fn set(&mut self, key: impl AsRef<str>, value: impl Into<String>) {
        let canon_key = canonical_header_key(key);
        self.remove_raw(&canon_key);
        self.add_owned(canon_key, value);
    }

    pub fn remove(&mut self, key: impl AsRef<str>) {
        let canon_key = canonical_header_key(key);
        self.remove_raw(&canon_key);
    }

    fn remove_raw(&mut self, key: impl AsRef<str>) {
        self.inner.retain(|h| h.0 != key.as_ref());
    }

    fn add_owned(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.push((key.into(), value.into()));
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = &'a Header;
    type IntoIter = Iter<'a, Header>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

fn canonical_header_key(key: impl AsRef<str>) -> String {
    let key = key.as_ref();
    let mut canon = String::with_capacity(key.len() + 1);

    for part in key.split("-") {
        for (ch_idx, ch) in part.chars().enumerate() {
            if ch_idx == 0 {
                canon.push(ch.to_ascii_uppercase());
            } else {
                canon.push(ch.to_ascii_lowercase());
            }
        }
        canon.push('-');
    }

    canon.pop();
    canon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut headers = Headers::new();

        headers.add("Content-Type", "text/plain");
        assert_eq!("text/plain", headers.get("content-type").unwrap());

        headers.add("accept", "text/plain");
        headers.add("Accept", "application/json");
        headers.add("ACCEPT", "text/html");
        assert_eq!(vec!["text/plain", "application/json", "text/html"], headers.values("Accept"));
        assert_eq!("text/plain", headers.get("accept").unwrap());

        headers.set("accept", "text/plain");
        assert_eq!(vec!["text/plain"], headers.values("Accept"));
        assert_eq!("text/plain", headers.get("accept").unwrap());

        headers.remove("accept");
        assert!(headers.values("accept").is_empty());
        assert!(headers.get("accept").is_none());
    }

    #[test]
    fn it_canonicalize_header_keys() {
        assert_eq!("Content-Type", canonical_header_key("content-type"));
        assert_eq!("Content-Type", canonical_header_key("CONTENT-TYPE"));
        assert_eq!("Content-Type", canonical_header_key("conTEnt-TyPe"));
        assert_eq!("X-Test-Header-Abc", canonical_header_key("x-test-header-abc"));
        assert_eq!("X-Test-Header-Abc", canonical_header_key("X-TEST-HEADER-ABC"));
        assert_eq!("X-Test-Header-Abc", canonical_header_key("X-TEST-header-abC"));
    }
}
