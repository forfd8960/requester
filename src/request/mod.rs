use std::collections::HashMap;

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct Response {
    pub status_code: u32,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u32, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }
}

impl Request {
    pub fn new(
        method: Method,
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

    pub fn send(&self) -> anyhow::Result<Response> {
        todo!()
    }
}

pub fn get() {}
