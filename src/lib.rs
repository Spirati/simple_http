//! Quick and dirty HTTP library for handling simple web requests

use std::io::prelude::*;
use regex::Regex;
use std::ops::Deref;
use http::{Request, Response};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

/// Function signature for handler functions 
pub type RequestHandler = fn(req: Request<&str>) -> Response<String>;

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
    /// 
    /// # Example
    /// 
    /// ```rust
    /// fn main() {
    ///     let app = App::new("127.0.0.1:8000");
    /// 
    ///     app.add_handler("/echo", echo_handler);
    /// }
    /// 
    /// // Responds to request with host uri
    /// fn echo_handler(req: http::Request<&str>) -> http::Response<String> {
    ///     let host = format!("{:?}", req.headers().get("Host").unwrap());
    ///     http::Response::builder()
    ///         .status(200)
    ///         .body(host)
    ///         .unwrap()
    /// }
    /// ```
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
                let cropped = http_util::parse_header(v);
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
    //! A collection of [`RequestHandler`]s useful for common, simple behaviors like 40x status codes

    /// Simple catch-all function for returning a 404 when no paths match
    pub fn not_found(_req: http::Request<&str>) -> http::Response<String> {
        http::Response::builder()
            .status(404)
            .body(String::new())
            .unwrap()
    }
}

pub mod http_util {
    use std::ops::Deref;

    /// Extract the value from a given header in a request
    /// 
    /// # Example
    /// 
    /// Given an HTTP request like
    /// ```
    /// GET /foo HTTP/1.1
    /// Host: bar.com 
    /// ```
    /// encoded in a [`http::Request`] `req`, we can extract `bar.com` from the `Host` header like so:
    /// ```rust
    /// let host: String = extract_header(req, "Host");
    /// ```
    pub fn extract_header<T>(src: http::Request<T>, header: &str) -> String {
        let hv = src.headers().get(header).unwrap();
        let cropped = String::from(format!("{:?}", hv));
        let cropped: std::borrow::Cow<'_, str> = regex::Regex::new("(^\")|(\"$)")
            .unwrap()
            .replace_all(cropped.as_str(), "");
        
        String::from(cropped.deref())
    }
    /// Extract the value from a given header from a [`http::HeaderValue`]
    /// 
    /// Example
    /// Given an HTTP request like
    /// ```
    /// GET /foo HTTP/1.1
    /// Host: bar.com 
    /// ```
    /// with the Host header encoded in a [`http::HeaderValue`] `hv` (e.g. from a [`http::Request`]'s `.headers().iter()`), we can extract `bar.com` like so:
    /// ```rust
    /// let host: String = parse_header(hv);
    /// ```
    pub fn parse_header(header: &http::HeaderValue) -> String {
        let cropped = String::from(format!("{:?}", header));
        let cropped: std::borrow::Cow<'_, str> = regex::Regex::new("(^\")|(\"$)")
            .unwrap()
            .replace_all(cropped.as_str(), "");
        
        String::from(cropped.deref())
    }
}