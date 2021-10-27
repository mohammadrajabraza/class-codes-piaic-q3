use std::{convert::Infallible,net::SocketAddr,error};
use hyper::{Body, Request, Response, Server,Method, StatusCode};
use hyper::Client;
use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt as _;

async fn shutdown_signal(){
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
async fn fetch_data() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    // Still inside `async fn main`...
let client = Client::new();

// Parse an `http::Uri`...
let uri = "http://httpbin.org/ip".parse()?;

// Await the response...
let resp = client.get(uri).await?;

println!("Response: {}", resp.status());
Ok(())

}

async fn echo(req: Request<Body>) -> Result<Response<Body>, Infallible>{

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo, /echo/uppercase, /reverse , ");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        (&Method::POST, "/echo/uppercase") => {
            // This is actually a new `futures::Stream`...
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
        (&Method::GET, "/echo/kuchbhi") => {
            *response.body_mut() = Body::from("Hello! this is the route from echo endpoint to a new 
            sub-endpoint as 'KUCH BHI' ");
        },
        (&Method::GET, "/fetchdata")=> {
           // *response.body_mut() = Body::from("This is for the Client!");
           println!("This is working");
            fetch_data().await;
        },
        (&Method::POST, "/reverse") => { 
            
            let word = hyper::body::to_bytes(req.into_body()).await?;
            let wordr = word.iter().rev().cloned().collect::<Vec<u8>>();
            
            Ok(Response::new(Body::from(wordr)))
        },
        (&Method::POST, "/count/chars") => { 

            let wordB = hyper::body::to_bytes(req.into_body()).await?;
            let countss = wordB.iter().cloned().collect::<Vec<u8>>(); 
              
            let mut countss = 0;
            for  c in wordB.chars(){
                if c == ' '{
                    countss += 1;
                }
            };
            countss = wordB.chars().count() - countss;
            let countss : String = countss.to_string();

            
            
            Ok(Response::new(Body::from(countss)))
        },                     

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("Requested resource not found at this End Point!");
        },
    };

    Ok(response)
        //Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:5000
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.

    let make_svc = make_service_fn(|_conn| async{
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");


    // Run this server for... forever!
    if let Err(e) = server.await{
        eprintln!("Server error: {}", e);
    }



}
