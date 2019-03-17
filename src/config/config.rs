use std::fs;
use std::error::Error;
use std::fmt::Display;

const DOCUMENT_ROOT_NAME: &str = "document_root";
const THREAD_LIMIT_NAME: &str = "thread_limit";

pub const DOCUMENT_ROOT_ERROR: &str = "Can't find document_root";
pub const DOCUMENT_ROOT_INVALID_FORMAT: &str = "Invalid document root format";

pub const THREAD_LIMIT_ERROR: &str = "Can't find thread_limit";
pub const THREAD_LIMIT_INVALID_FORMAT: &str = "Invalid thread limit format";

#[derive(Debug)]
pub struct Config {
    pub thread_count: u16,
    pub dir_root: String,
}

impl Config {

    pub fn read(path: &str) -> Result<Config, String> {
        let file_string = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Error: {}", err)),
        };

        match Config::parse(file_string) {
            Ok(cfg) => return Ok(cfg),
            Err(err) => return Err(err),
        };
    }

    fn parse(raw: String) -> Result<Config, String> {
        let params: Vec<_> = raw.split("\n").collect();

        let document_root_pair: Vec<_> = match params.iter().find(|&x| x.to_string().contains(DOCUMENT_ROOT_NAME)) {
            Some(pair) => pair.trim().split(" ").collect(),
            None => return Err(String::from(DOCUMENT_ROOT_ERROR)),
        };

        if (document_root_pair.len() != 2) {
            return Err(String::from(DOCUMENT_ROOT_INVALID_FORMAT));
        }

        let thread_limit_pair: Vec<_> = match params.iter().find(|&x| x.to_string().contains(THREAD_LIMIT_NAME)) {
            Some(pair) => pair.trim().split(" ").collect(),
            None => return Err(String::from(THREAD_LIMIT_ERROR)),
        };

        if (thread_limit_pair.len() != 2) {
            return Err(String::from(THREAD_LIMIT_INVALID_FORMAT));
        }

        Ok(Config{
            thread_count: thread_limit_pair[1].parse().unwrap(),
            dir_root: String::from(document_root_pair[1]),
        })
    }
}