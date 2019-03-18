use crate::thread_pool::thread_pool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use crate::http::request::HTTPRequest;
use crate::http::response::HTTPResponse;
use std::time::Duration;
use std::collections::{HashMap};
use std::fs::File;
use std::path::Path;

pub struct Server {
    thread_pool: ThreadPool,
    listener: TcpListener,
    dir_root: Arc<Mutex<String>>,
}

impl Server {
    pub fn new(root: String, thread_count: u16, adress: String, port: String) -> Server {
        let full_address = format!("{}:{}", adress, port);
        
        let listener = match TcpListener::bind(&full_address) {
            Ok(lst) => lst,
            Err(err) => panic!(err),
        };

        println!("Start tcp listener: {}", full_address);

        Server{
            thread_pool: ThreadPool::new(thread_count as usize),
            listener: listener,
            dir_root: Arc::new(Mutex::new(root)),
        }
    }

    pub fn start(&self) {
        println!("Start server");

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            let root = self.dir_root.clone();
            self.thread_pool.execute(move|| {
                let root_dir_guard = root.lock().unwrap();

                let root_dir = (*root_dir_guard).clone();

                std::mem::drop(root_dir_guard);

                Server::handle_connection(stream, &root_dir);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, root_dir: &String) {
        let mut buffer = [0; 512];

        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => return,
            Ok(_) => {},
            Err(err) => {
                println!("Error while read: {}", err);
                return;
            } 
        }

        let request = match HTTPRequest::parse(&mut buffer) {
            Ok(req) => req,
            Err(()) => {
                let mut resp = Server::handle_bad_request();
                resp.setDate();
                resp.setServer("Rust (Unix)");
                resp.setConnection("close");
                resp.send(&stream);
                return;
            }
        };

        let mut resp = match Server::handle_request(request, &root_dir) {
            Ok(resp) => resp,
            Err(()) => {
                println!("Error handle request");
                return;
            }
        }; 

        resp.setDate();
        resp.setServer("Rust (Unix)");
        resp.setConnection("close");

        resp.send(&stream);
    }

    fn handle_request(req: HTTPRequest, root: &String) -> Result<HTTPResponse, ()> {
        let path = req.path;
        let method = req.method;
        println!("{}{}",&root, &path);
        let resp = match &method[..] {
            "GET" => Server::handle_get(path, &root, req.isAutoIndex),
            "HEAD" => Server::handle_head(path, &root, req.isAutoIndex),
            _ => Server::handle_other(),
        };

        Ok(resp)
    }

    fn handle_bad_request() -> HTTPResponse {
        println!("Handle bad");
        let mut resp = HTTPResponse::new();

        resp.setBadRequest();
        return resp;
    }

    fn handle_get(path: String, root: &String, isAutoIndex: bool) -> HTTPResponse {
        let path = format!("{}{}", root, path);
        let mut resp = HTTPResponse::new();

        match File::open(&path) {
            Ok(file) => {
                let p = Path::new(&path);
                resp.setContentType(&p);
                resp.setContentLength(&p);
                resp.setOk(Some(file));
            },
            Err(err) => {
                if (isAutoIndex) {
                    resp.set403()
                } else {
                    resp.setNotFound();
                }
            }
        };

        return resp;
    }

    fn handle_head(path: String, root: &String, isAutoIndex: bool) -> HTTPResponse {
        let path = format!("{}{}", root, path);
        
        let mut resp = HTTPResponse::new();

        let p = Path::new(&path);

        match p.exists() {
            true => {
                resp.setContentType(&p);
                resp.setContentLength(&p);
                resp.setOk(None);
            },
            false => {
                if (isAutoIndex) {
                    resp.set403()
                } else {
                    resp.setNotFound();
                }
            }
        };

        return resp;
    }

    fn handle_other() -> HTTPResponse {
        println!("Handle other");
        let mut resp = HTTPResponse::new();

        resp.setNotAllowed();
        return resp;
    }
}