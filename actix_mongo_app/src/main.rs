use actix_web::{
    web, App, HttpResponse, HttpServer, Result,
};

use mongodb::{Client, options::ClientOptions};

use dotenv::dotenv;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    println!("Actix-web server started at {:?}", env::var("SOCKET_ADDR").unwrap());
    
    let mut client_options = ClientOptions::parse(env::var("DB_URL").unwrap()).await.expect("Mongo error");
    client_options.app_name = Some("Actix-Mongo-App".to_string()); //Optional
    let client = Client::with_options(client_options).unwrap();

    // Alternatively you can use the line below instead of above 3 lines
    // let client = Client::with_uri_str(env::var("DB_URL").unwrap()).await.expect("Mongo error");

    // Checking database connectivity
    for db_name in client.list_database_names(None, None).await.expect("Mongo error") {
        println!("{}", db_name);
    }

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/add").route(web::get().to(add)))
            .service(web::resource("/update").route(web::get().to(update)))
            .service(web::resource("/delete").route(web::get().to(delete)))
            .service(web::resource("/view").route(web::get().to(view)))
            
    })
    .bind(env::var("SOCKET_ADDR").unwrap())?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!(
                "Hello from actix server!"
            )
        ))
}


async fn add() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!(
                "Following record(s) added"
            )
        ))
}

async fn update() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!(
                "Following record(s) updated"
            )
        ))
}

async fn delete() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!(
                "Following record(s) deleted"
            )
        ))
}

async fn view() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!(
                "Records"
            )
        ))
}
