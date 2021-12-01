mod lib;
use lib::App;

use http::{Request, Response};

fn main() {
    let mut app = App::new("127.0.0.1:7878");
    app.add_handler(String::from("/test"), test_url);
    app.add_handler(String::from("/echo"), echo_handler);
    app.run();
}

fn test_url(_req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(String::from("Hello, world!"))
        .unwrap()
}

fn echo_handler(req: http::Request<&str>) -> http::Response<String> {
    let host = lib::http_util::extract_header(req, "Host");
    http::Response::builder()
        .status(200)
        .body(host)
        .unwrap()
}