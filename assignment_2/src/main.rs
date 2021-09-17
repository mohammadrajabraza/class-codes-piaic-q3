use futures::TryStreamExt as _;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try posting data to /echo, /echo/to_uppercase, /reverse or /count/chars",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/to_uppercase") => {
            let mapping = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(mapping)))
        }
        // For reversing text before sending back to client using a stream.
        (&Method::POST, "/reverse") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let revers = full_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(revers)))
        }

        // To post character count without space.
        (&Method::POST, "/count/chars") => {
            let body = hyper::body::to_bytes(req.into_body()).await?;
            let mut chars = body.iter().cloned().collect::<Vec<u8>>();
            // let mut str = std::env::args().chars;
            let str = unsafe { std::str::from_utf8_unchecked_mut(&mut chars) };

            let result_chars_count = count_nonspace_chars(&str);
            Ok(Response::new(Body::from(result_chars_count)))
        }
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    // creates a socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // creates service from our `echo` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever and terminate it gracefully!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
fn count_nonspace_chars(test: &str) -> String {
    let mut result = 0;
    for c in test.chars() {
        // Count all chars that are not whitespace.
        if !c.is_whitespace() {
            result += 1;
        }
    }
    let result_str: String = result.to_string();
    return result_str;
}
