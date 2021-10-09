use actix_web::{web, App, HttpResponse, HttpServer, Result};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/products").route(web::get().to(products)))
            .service(web::resource("/about").route(web::get().to(about)))
            .service(web::resource("/contact").route(web::get().to(contact)))
            .service(web::resource("/settings").route(web::get().to(settings)))
    })
    .bind("127.0.0.1:5002")?
    .run()
    .await  
}

async fn index() -> Result<HttpResponse> {
    // This will render index.html in response
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
    )
}

async fn products() -> Result<HttpResponse> {
    // This will render products.html in response
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/products.html"))
    )
}

async fn about() -> Result<HttpResponse> {
    // This will render about.html in response
    Ok(HttpResponse::Ok()
        .content_type("text/html;image/jpg;image/png; charset=utf-8")
        .body(include_str!("../static/about.html"))
    )
}

async fn contact() -> Result<HttpResponse> {
    // This will render contact.html in response
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/contact.html"))
    )
}

async fn settings() -> Result<HttpResponse> {
    // This will render settings.html in response
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/settings.html"))
    )
}
