use std::collections::HashMap;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    BAD,
}

pub struct HttpEntity {
    pub http_version: String,
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}
