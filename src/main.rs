mod lib;
use lib::App;

use http::{Request, Response};

fn main() {
    let mut app = App::new("127.0.0.1:7878");
    app.add_handler(String::from("/test"), test_url);

    app.run();
}

fn test_url(_req: Request<&str>) -> Response<&str> {
    Response::builder()
        .status(200)
        .body("Hello, world!")
        .unwrap()
}