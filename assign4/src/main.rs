// step 2
use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

// Step 3
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Step 4
    HttpServer::new(|| {

        // Step 6
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/about").route(web::get().to(about)))
            .service(web::resource("/products").route(web::get().to(products)))
            .service(web::resource("/contact").route(web::get().to(contact)))
            .service(web::resource("/profile").route(web::get().to(profile)))
            .service(web::resource("/settings").route(web::get().to(settings)))
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


// Step 5
async fn index() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

async fn about() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/about.html")))
}

async fn products() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/products.html")))
}

async fn contact() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/contact.html")))
}

async fn profile() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/profile.html")))
}

async fn settings() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/settings.html")))
}

