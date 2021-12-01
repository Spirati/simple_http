//! Quick and dirty HTTP library for handling simple web requests

use std::io::prelude::*;
use regex::Regex;
use std::ops::Deref;
use http::{Request, Response};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

/// Function signature for handler functions 
pub type RequestHandler = fn(req: Request<&str>) -> Response<&str>;

/// A HashMap that associates paths with [`RequestHandler`] functions
pub type HandlerMap = HashMap<String, RequestHandler>;

/// Manages [`RequestHandler`] functions and the state of your web service
pub struct App {
    listener: TcpListener,
    handlers: HandlerMap
}

impl App {
    /// Creates a new [`App`] bound to `ip`
    pub fn new(ip: &str) -> App {
        App{listener: TcpListener::bind(ip).unwrap(), handlers: HandlerMap::new()}
    }
    /// Start the [`App`] instance and start listening for incoming connections
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            
            self.handle_connection(stream);
        }
    }
    /// Add a [`RequestHandler`] that handles requests to a path matching `path`
    pub fn add_handler(&mut self, path: String, handler: RequestHandler) {
        self.handlers.insert(path, handler);
    }
    fn parse_request(req_str: &str) -> Request<&str> {
        let re = Regex::new(concat!(
            r"(?m)(?P<method>[A-Z]+) ",
            r"(?P<path>[^ ]+) ",
            r"HTTP/1\.\d\r\n",
            r"(?P<headers>(?:[A-Za-z-]+: [^\r\n]+\r\n)+)",
            r"(?:\r\n(?P<body>.+))*"
        )).unwrap();
        let caps = re.captures(req_str).unwrap();
    
        let method = caps.name("method").unwrap().as_str();
        let path = caps.name("path").unwrap().as_str();
        let headers = match caps.name("headers") {
            Option::Some(t) => t.as_str(),
            Option::None => ""
        };
    
        let headers: Vec<(&str, &str)> = headers
            .split("\r\n")
            .filter(|x| x != &"")
            .map(|x| x.split_once(": ").unwrap())
            .collect();
            
        let body = match caps.name("body") {
            Option::Some(t) => t.as_str(),
            Option::None => ""
        };
    
        let mut build = Request::builder()
            .method(method)
            .uri(path);
        for (key, value) in headers {
            build = build.header(key, value);
        }
        build.body(body).unwrap()
    }
    fn handle_connection(&self, mut stream: TcpStream) -> Option<usize> {
        let mut buffer = [0; 1024];
    
        stream.read(&mut buffer).unwrap();
    
        let request_string = String::from_utf8_lossy(&buffer[..]);
    
        let req = App::parse_request(request_string.deref());
        let res = match self.handlers.get(req.uri().path()) {
            Some(t) => t(req),
            None => default_handlers::not_found(req)
        };

        let mut header_string = String::new();
    
        res.headers().iter().for_each(
            |(k, v)| {
                let cropped = String::from(format!("{:?}", v));
                let cropped: std::borrow::Cow<'_, str> = Regex::new("(^\")|(\"$)")
                    .unwrap()
                    .replace_all(cropped.as_str(), "");
                header_string = format!(
                    "{}{}\r\n", 
                    header_string, format!("{}: {}", k, cropped.deref())
                )
            }
        );
    
        let res_str = format!(
            "{version:?} {status} {reason}\r\n{headers}\r\n{body}",
            version = res.version(),
            status = res.status().as_str(),
            reason = res.status().canonical_reason().unwrap(),
            headers = header_string,
            body = res.body()
        );
    
        match stream.write(String::from(res_str).as_bytes()) {
            Ok(t) => Some(t),
            Err(_) => None
        }
    }
}

mod default_handlers {
    /// Simple catch-all function for returning a 404 when no paths match
    pub fn not_found(_req: http::Request<&str>) -> http::Response<&str> {
        http::Response::builder()
            .status(404)
            .body("")
            .unwrap()
    }
}