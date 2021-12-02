use simple_http::{http_util, app::App};
use http::{Request, Response};

pub fn echo_example() {
    let mut app = App::new("127.0.0.1:7878");

    app.add_handler("/echo", echo_handler);
    app.run();
}

fn echo_handler(req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(http_util::extract_header(req, "Host"))
        .unwrap()
}
