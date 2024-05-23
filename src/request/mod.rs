use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        Self {
            method,
            url,
            headers,
            body,
        }
    }
}

pub fn get() {}
