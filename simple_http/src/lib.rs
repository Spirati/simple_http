//! Quick and dirty HTTP library for handling simple web requests
use http::{Request, Response};
use regex::Regex;

/// Function signature for handler functions 
pub type RequestHandler = fn(req: Request<&str>) -> Response<String>;

/// A struct containing a pattern for URI paths to match; TBA is a means of extracting parameters (e.g. `/users/{id}`)
pub struct PathMatcher {
    pub regex: Regex
}

/// A `Vec` that associates [`PathMatcher`] expressions with [`RequestHandler`] functions
pub type HandlerList = Vec<(PathMatcher, RequestHandler)>;

pub mod app;
pub mod default_handlers;
pub mod http_util;