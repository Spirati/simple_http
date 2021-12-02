use simple_http::app::App;
use http::{Request, Response};

fn main() {
    let mut app = App::new("127.0.0.1:7878");

    app.add_handler("/", hello_handler);
    app.run();
}

fn hello_handler(_req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(String::from("Hello, world!"))
        .unwrap()
}
