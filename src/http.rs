extern crate hyper;

use rand::prelude::*;
use rand::distributions::Normal;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

fn generate_reading(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    let mut rng = thread_rng();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/temperature") => {
            if rng.gen_range(0, 4) > 2 {
                println!("Sending 500 error");
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *response.body_mut() = Body::from("ERROR");
            } else {
                let t = rng.sample(Normal::new(68.0, 4.0));

                *response.body_mut() =
                    Body::from(format!("{{\n\t\"farenheit\":\"{:.*e}\"\n}}", 3, t));
            }
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    response
}

pub fn simple_http_server() {
    let addr = ([127, 0, 0, 1], 30000).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn_ok(generate_reading))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Starting HTTP server on http://{}", addr);

    hyper::rt::run(server);
}
