extern crate percent_encoding;
use self::percent_encoding::percent_decode;

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: String,    
    pub path: String,
}

impl HTTPRequest {
    pub fn new() -> HTTPRequest {
        HTTPRequest {
            method: String::new(),
            path: String::new(),
        }
    }

    pub fn parse(buffer: &[u8]) -> Result<(HTTPRequest), ()> {
        let stringRaw = std::str::from_utf8(buffer).unwrap();
        let firstLine = stringRaw.split("\r\n").nth(0).unwrap();

        if firstLine.len() == 0 {
            return Err(());
        }

        let requestVec: Vec<&str> = firstLine.split(" ").collect();

        if requestVec.len() < 3 {
            return Err(());
        }

        Ok(HTTPRequest{
            method: match requestVec[0] {
                "GET" => String::from("GET"),
                "HEAD" => String::from("HEAD"),
                _ => return Err(()),
            },
            path: match parsePath(requestVec[1]) {
                Ok(p) => p,
                Err(()) => return Err(()),
            },
        })
    }
}

fn parsePath(path: &str) -> Result<String, ()> {
    let rawPath = path.split("?").nth(0).unwrap(); 
    
    match rawPath.contains("/..") {
        true => return Err(()),
        false => {},
    };

    let decoded = percent_decode(rawPath.as_bytes()).decode_utf8().unwrap();
    Ok(match decoded.split("/").last().unwrap() {
        "" => format!("{}{}", decoded, "index.html"),
        _ => String::from(decoded),
    })
}