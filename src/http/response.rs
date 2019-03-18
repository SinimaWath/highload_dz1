extern crate chrono;

use std::collections::{HashMap};
use std::net::{TcpStream};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::string::String;
use chrono::{DateTime, TimeZone, NaiveDateTime, UTC};

const HTTP_VERSION: &str = "HTTP/1.1";
const HTTP_TERMINATOR: &str = "\r\n";

pub struct HTTPResponse {
    pub headers: HashMap<String, String>,
    status: u16,
    pub file: Option<File>,
}

impl HTTPResponse {
    pub fn new() -> HTTPResponse{
        HTTPResponse{
            headers: HashMap::new(),
            status: 0,
            file: None,
        }
    }
    pub fn send(self, mut stream: &TcpStream) {
        let mut response = String::new();
        response.push_str(HTTP_VERSION);
        response.push_str(" ");
        response.push_str(&self.status.to_string());
        response.push_str(HTTP_TERMINATOR);

        for (header, value) in &self.headers {
            println!("{}:{}", header, value);
            response.push_str(format!("{}: {}", header, value).as_str());
            response.push_str(HTTP_TERMINATOR);
        }
        response.push_str(HTTP_TERMINATOR);

        println!("{}", response);
        stream.write(response.as_bytes()).unwrap();

        match self.file {
            Some(mut f) => {
                let mut buf = [0; 1024 * 1024];
                let mut n: u64 = 0;
                loop {
                    match f.read(&mut buf).unwrap() {
                        0 => { 
                            break; 
                        },
                        i => {
                            n += i as u64;
                            stream.write(&buf[..i]).unwrap();
                            f.seek(SeekFrom::Start(n as u64));
                        }
                    }
                }
            }
            None => {}
        }
        stream.flush().unwrap();
    }

    pub fn setContentType(&mut self, path: &Path) {
        let ext = match path.extension() {
            Some(ext) => ext,
            None => panic!("no ext")
        };
        let content_type = HTTPResponse::get_content_type_by_ext(ext.to_str().unwrap());
        self.push_header("Content-Type".to_owned(), content_type);
    }

    fn get_content_type_by_ext(ext: &str) -> String{
        match ext {
            "html" => String::from("text/html"),
            "css" => String::from("text/css"),
            "js" => String::from("application/javascript"),
            "jpeg" | "jpg" => String::from("image/jpeg"),
            "png" => String::from("image/png"),
            "swf" => String::from("application/x-shockwave-flash"),
            _ => String::from("application/chiki_briki"),
        }
    }

    pub fn setContentLength(&mut self, path: &Path) {

        let content_len = match path.metadata() {
            Ok(meta) => meta.len(),
            Err(err) => panic!("Metadat err: {}", err),
        };

        self.push_header("Content-Length".to_owned(), format!("{}",content_len));
    }

    pub fn push_header(&mut self, header: String, value: String) {
        self.headers.insert(header, value);
    }

    pub fn setOk(&mut self, file: Option<File>) {
        match file {
            Some(file) => self.file = Some(file),
            None => self.file = None,
        }

        self.status = 200;
    }
    
    pub fn setNotFound(&mut self) {
        self.file = None;
        self.status = 404;
    }

    pub fn setNotAllowed(&mut self) {
        self.file = None;
        self.status = 405;
    }

    pub fn setDate(&mut self) {
        let utc = UTC::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        self.push_header("Date".to_owned(), utc);
    }

    pub fn setServer(&mut self, server: &str) {
        self.push_header("Server".to_owned(), server.to_owned());
    }

    pub fn setConnection(&mut self, conn: &str) {
        self.push_header("Connection".to_owned(), conn.to_owned());
    }
}