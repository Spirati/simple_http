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
    }}
