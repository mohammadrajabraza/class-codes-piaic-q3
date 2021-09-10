use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;

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
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }
        // For reversing text before sending back to client using a stream.
        (&Method::POST, "/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // To post character count without space.
        (&Method::POST, "/count/chars") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let chars_count = whole_body.iter().cloned().collect::<Vec<u8>>();
            let _str = match std::str::from_utf8(&chars_count) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let result_chars_count = count_nonspace_chars(_str);
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

fn count_nonspace_chars(test: &str) -> String {

    let mut result = 0;
    for c in test.chars() {
        // Count all chars that are not whitespace.
        if !c.is_whitespace() {
            result += 1;
        }
    }
    let result_st: String = result.to_string();
    return result_st;
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