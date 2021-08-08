use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;

#[tokio::main]
async fn main() {
    // Binding server to 127.0.0.1:3000 to respond to requests
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // let addr = SocketAddrhello_world::from(([192, 168, 1, 102], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
// ==>  
        /* Ok::<_, Infallible>(service_fn(hello_world)) */
        Ok::<_, Infallible>(service_fn(echo_service))
    });

    // Configuring server
    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// ==> 
/*
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}
*/

async fn echo_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        // root route to instruct users the usage of endpoints
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from
            ("Try POSTing data to /echo\nOR Try POSTing data to /echo/uppercase");
        },
        // Route to handle requests that take input from body and 
        // response back text 
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        // Route to handle requests that take input from body and 
        // send response converted body text into Upper case 
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req
            .into_body()
            .map_ok(|chunk| {
                chunk.iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
    
            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        // Fallback route to handle all other requests other than specified above
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("Requested resource not found at this endpoint!");
        },
    };

    Ok(response)
}