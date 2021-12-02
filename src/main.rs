mod lib;
use lib::app::App;

use http::{Request, Response};

fn main() {
    let mut app = App::new("127.0.0.1:7878");
    app.add_handler("/uwu?", uwu_handler);
    app.add_handler("/echo", echo_handler);
    app.add_handler("/.*", catchall_handler);
    app.run();
}

fn uwu_handler(req: Request<&str>) -> Response<String> {

    let uwu = match lib::http_util::parse_query(req) {
        Some(t) => t.replace("r", "w").replace("l", "w"),
        None => String::from("Couwdn't pawse that input ÒwÓ")
    };

    Response::builder()
        .status(200)
        .body(uwu)
        .unwrap()
}

fn catchall_handler(_req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(String::from("Purgatory..."))
        .unwrap()
}

fn echo_handler(req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(lib::http_util::extract_header(req, "Host"))
        .unwrap()
}
