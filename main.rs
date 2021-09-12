//#![deny(warnings)]
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(run))
    });

    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}


async fn run(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
let mut response = Response::new(Body::empty());

match (req.method(), req.uri().path()) {
    (&Method::POST, "/echo/reverse") => {
        let body = hyper::body::to_bytes(req.into_body()).await?;
        let reverse_it = body.iter().rev().cloned().collect::<Vec<u8>>();
        *response.body_mut() = reverse_it.into();
    },
    (&Method::POST, "/echo/count/chars") => {
        let body = hyper::body::to_bytes(req.into_body()).await?;
        let count_it = body.iter().count().to_string();
        *response.body_mut() = count_it.into();
    },
        _ => {
        *response.status_mut() = StatusCode::NOT_FOUND;
    },
};
Ok(response)
}
