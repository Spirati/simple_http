mod lib;
use lib::app::App;

use http::{Request, Response};

fn main() {
    let mut app = App::new("127.0.0.1:7878");
    app.add_handler("/echo", echo_handler);
    app.run();
}

fn echo_handler(req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(lib::http_util::extract_header(req, "Host"))
        .unwrap()
}
