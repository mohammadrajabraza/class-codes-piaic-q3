// step 2
use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
// Step 8
use dotenv::dotenv;
use std::env;

// Step 3
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Step 9
    dotenv().ok();
    // Step 4
    HttpServer::new(|| {

        // Step 6
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/hello").route(web::get().to(hello)))
            
    })
    // Step 10
    .bind(env::var("SOCKET_ADDR").unwrap())?
    .run()
    .await
}

// Step 5
async fn index() -> Result<HttpResponse> {
    
    // This will simply render text in response
    // Ok(HttpResponse::Ok()
    //     .content_type("text/plain")
    //     .body(format!(
        //         "Hello from actix server!"
        //     )
        // ))

    // This will render index.html in response
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

async fn hello() -> Result<HttpResponse> {
    // Ok(HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(include_str!("../static/form.html")))
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "Hello User!"
    )))
}