use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use hyper::Client;
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

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }
    
    // // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

// ==> 
/*
async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}
*/

async fn echo_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    
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
        (&Method::GET, "/fetchdata") => {
            println!("working.....");
            fetch_data().await;
            // *response.body_mut() = Body::from(resp.status());
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
        // Route to handle requests that take input from body and 
        // send response converted body text as Reverse string 
        (&Method::POST, "/reverse") => {

            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // Iterate the full body in reverse order and collect into a new Vec.
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();
        
            *response.body_mut() = reversed.into();
        },
        // Route to handle requests that take input from body and 
        // send response converted body text as Reverse string 
        (&Method::POST, "/count/chars") => {
            let mut char_count = 0;
            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            let mut full_body_iter = full_body.iter();

            for c in full_body_iter{
                if *c != 32 {
                    char_count = char_count + 1;
                }                
            }

            //let char_count = filter_body.count();
            // Use `Body::wrap_stream` to convert it to a `Body`...
            *response.body_mut() = char_count.to_string().into();
        },
        // Fallback route to handle all other requests other than specified above
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("Requested resource not found at this endpoint!");
        },
    };

    Ok(response)
}

async fn fetch_data() -> Result<(), Box<dyn std::error::Error>> {
    
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse()?;
    let resp = client.get(uri).await?;
    println!("Response: {:?}", resp.body());
    
    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
