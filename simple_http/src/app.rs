//! A module to encapsulate the functionality of the [`App`] struct

use super::{HandlerList, RequestHandler};
use regex::Regex;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
/// Manages [`RequestHandler`] functions and the state of your web service
pub struct App {
    listener: TcpListener,
    handlers: HandlerList,
}

impl App {
    /// Creates a new [`App`] bound to `ip`
    pub fn new(ip: &str) -> App {
        App {
            listener: TcpListener::bind(ip).unwrap(),
            handlers: HandlerList::new(),
        }
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
    /// Path matching is done via regular expressions: a `path` of `"/(foo|bar)"` would match both `/foo` and `/bar`.
    /// Take note that matching is done lazily in order of creation: less specific patterns should be added after
    /// more specific ones in order to avoid short-circuiting behavior.
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
    pub fn add_handler(&mut self, path: &str, handler: RequestHandler) {
        self.handlers.push((
            super::PathMatcher {
                regex: Regex::new(path).unwrap(),
            },
            handler,
        ));
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Option<usize> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request_string = String::from_utf8_lossy(&buffer[..]);

        let req = super::http_util::parse_request(request_string.deref()).unwrap();
        let matching_path = self
            .handlers
            .iter()
            .find(|r| r.0.regex.is_match(req.uri().path()));

        let res = match matching_path {
            Some((_, h)) => h(req),
            None => super::default_handlers::not_found(req),
        };

        let mut header_string = String::new();

        res.headers().iter().for_each(|(k, v)| {
            let cropped = super::http_util::parse_header(v);
            header_string = format!(
                "{}{}\r\n",
                header_string,
                format!("{}: {}", k, cropped.deref())
            )
        });

        let res_str = format!(
            "{version:?} {status} {reason}\r\n{headers}\r\n{body}",
            version = res.version(),
            status = res.status().as_str(),
            reason = res.status().canonical_reason().unwrap(),
            headers = header_string,
            body = res.body()
        );

        match stream.write(res_str.as_bytes()) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }
}
