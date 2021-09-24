use mongodb::{Client, options::ClientOptions};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb+srv://<UserName>:<Password>@sandbox.xevcm.mongodb.net/example?retryWrites=true&w=majority").await?;
    client_options.app_name = Some("Rust-Mongo-App".to_string());
    let client = Client::with_options(client_options)?;
    
    // Checking database connectivity
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // Get a handle to a database.
    let db = client.database("example");

    // List the names of the collections in that database.
    // for collection_name in db.list_collection_names(None).await? {
    //     println!("{}", collection_name);
    // }


    Ok(())
}   
