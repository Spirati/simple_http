use simple_http::app::App;
use http::{Request, Response};

pub fn path_example() {
    let mut app = App::new("127.0.0.1:7878");
    
    app.add_handler("/(foo|bar)", foobar_handler);
    app.run();
}

fn foobar_handler(req: Request<&str>) -> Response<String> {
    Response::builder()
        .status(200)
        .body(String::from(req.uri().path()))
        .unwrap()
}
