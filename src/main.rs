use dz1::http::request;
use std::str;
use std::iter;
use std::str::FromStr;

fn main() {
    let s = String::from("HEAD /foo/bar/ HTTP/1.1
Host: example.org");
    let req = request::HTTPRequest::parse(s.as_bytes());
    match req {
        Ok(req) => println!("{:?}", req),
        Err(()) => println!("Error"),
    };
}

// fn main() {
//     let s = "/foo/bar/".as_bytes();
//     s.as_bytes();
// }