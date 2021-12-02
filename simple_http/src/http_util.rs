//! A collection of useful operations on `[http]` structs
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
    let cropped = format!("{:?}", hv);
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
    let cropped = format!("{:?}", header);
    let cropped: std::borrow::Cow<'_, str> = regex::Regex::new("(^\")|(\"$)")
        .unwrap()
        .replace_all(cropped.as_str(), "");

    String::from(cropped.deref())
}
/// Remove percent formatting from a [`http::Request`]
pub fn parse_query(req: http::Request<&str>) -> Option<String> {
    match req.uri().query() {
        Some(t) => {
            let decoded_query = percent_encoding::percent_decode_str(t).decode_utf8_lossy();
            Some(String::from(decoded_query))
        }
        None => None,
    }
}

/// Take a raw HTTP request string and create a [`http::Request`] with the relevant fields
pub fn parse_request(req_str: &str) -> Option<http::Request<&str>> {
    let re = regex::Regex::new(concat!(
        r"(?m)(?P<method>[A-Z]+) ",
        r"(?P<path>[^ ]+) ",
        r"HTTP/1\.\d\r\n",
        r"(?P<headers>(?:[A-Za-z-]+: [^\r\n]+\r\n)+)?",
        r"(?:\r\n(?P<body>.+))?"
    ))
    .unwrap();
    let caps = match re.captures(req_str) {
        Some(t) => t,
        None => {
            println!("{}", req_str);
            return Option::None;
        }
    };

    let method = caps.name("method").unwrap().as_str();
    let path = caps.name("path").unwrap().as_str();
    let headers = match caps.name("headers") {
        Option::Some(t) => t.as_str(),
        Option::None => "",
    };

    let headers: Vec<(&str, &str)> = headers
        .split("\r\n")
        .filter(|x| x != &"")
        .map(|x| x.split_once(": ").unwrap())
        .collect();

    let body = match caps.name("body") {
        Option::Some(t) => t.as_str(),
        Option::None => "",
    };

    let mut build = http::Request::builder().method(method).uri(path);
    for (key, value) in headers {
        build = build.header(key, value);
    }
    Some(build.body(body).unwrap())
}

pub fn construct_request(req: http::Request<&str>) -> String {
    format!(
        "{method} {path} {version:?}\r\nHost: {host:?}\r\n{headers}\r\n{body}",
        method = req.method(),
        path = req.uri().path(),
        version = req.version(),
        host = req.uri().host().unwrap(),
        headers = req
            .headers()
            .iter()
            .map(
                |hv| format!("{}: {}", hv.0.as_str(), parse_header(hv.1))
            )
            .collect::<Vec<String>>()
            .join("\r\n"),
        body = req.body()
    )
}