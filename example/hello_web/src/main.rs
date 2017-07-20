extern crate tiny_http;

fn main() {
    use tiny_http::{Server, Response};

    let bind = "0.0.0.0:8000";

    let server = Server::http(bind).unwrap();

    println!("Server started bind on {}", bind);

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                 request.method(),
                 request.url(),
                 request.headers()
        );

        let response = Response::from_string("hello world");
        request.respond(response);
    }
}