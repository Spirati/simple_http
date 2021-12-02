//! A collection of [`RequestHandler`](super::RequestHandler)s useful for common, simple behaviors like 40x status codes

/// Simple catch-all function for returning a `404 Not Found` when no paths match
pub fn not_found(_req: http::Request<&str>) -> http::Response<String> {
    http::Response::builder()
        .status(404)
        .body(String::new())
        .unwrap()
}