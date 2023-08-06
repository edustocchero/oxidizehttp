use std::{collections::HashMap, fmt::Debug};

/// An enum containing the http methods.
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    BAD,
}

/// An enum containing the supported http versions.
#[derive(Debug)]
pub enum HttpVsn {
    HTTP1_1,
}

/// The http structure.
pub struct HttpEntity {
    pub http_version: HttpVsn,
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Debug for HttpEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpEntity")
            .field("http_version", &self.http_version)
            .field("method", &self.method)
            .field("path", &self.path)
            .field("headers", &self.headers)
            .finish()
    }
}
