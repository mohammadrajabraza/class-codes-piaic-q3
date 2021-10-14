// step 2
use actix_web::{
    web, App, HttpResponse, HttpServer, Result,
};
// Step 8
use dotenv::dotenv;
use std::env;
// step 12
use actix_files as fs;

// Step 3
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting actix-web server");
    // Step 9
    dotenv().ok();
    // Step 4
    HttpServer::new(|| {

        // Step 6
        App::new()
            .service(web::resource("/about").route(web::get().to(about)))
            .service(web::resource("/services").route(web::get().to(services)))
            .service(web::resource("/products").route(web::get().to(products)))
            .service(web::resource("/contact").route(web::get().to(contact)))
            .service(web::resource("/hello").route(web::get().to(hello)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(fs::Files::new("/static", "static/").show_files_listing())
            
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

async fn about() -> Result<HttpResponse> {
    println!("User visited about page");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/about.html")))
}

async fn services() -> Result<HttpResponse> {
    println!("User visited services page");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/services.html")))
}

async fn products() -> Result<HttpResponse> {
    println!("User visited products page");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/products.html")))
}

async fn contact() -> Result<HttpResponse> {
    println!("User visited contact page");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/contact.html")))
}

async fn hello() -> Result<HttpResponse> {
    // Ok(HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(include_str!("../static/form.html")))
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!(
        "Hello User!"
    )))
}
