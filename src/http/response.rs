use std::collections::{HashMap};

const HTTP_VERSION: &str = "HTTP/1.1";
const HTTP_TERMINATOR: &str = "\r\n";

pub struct HTTPResponse {
    headers: HashMap<String, String>,
    status: u16,
}