use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

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

    let collection = db.collection::<Product>("products");
    // let collection = db.collection("test");

    // let doc = Product { 
    //     item: "Tablet".to_string(), 
    //     qty : 23, 
    //     status: "C"
    // };
    // let doc = doc! { "item": "Tablet", "qty" : 23};
    // let record = collection.insert_one(doc, None).await?;
    // println!("Newly inserted record : {:?}", record);
    
    
    // Inserting multiple records
    // let docs = vec![
    //     doc! { "_id": 00000011 , "item": "Camera", "qty": 23, "status": "B", "size": { "h": 10, "w": 33, "uom": "mm"} },
    //     doc! { "_id": 00000012 , "item": "Air pods", "qty": 23, "status": "B", "size": { "h": 10, "w": 33, "uom": "mm"} },
    //     doc! { "_id": 00000013 , "item": "Wireless Charger", "qty": 23, "status": "B", "tags": ["Blue", "Green"] },
    // ];
    // let docs = vec![
    //     doc! { "item": "Camera", "qty": 23, "status": "B", "size": { "h": 10, "w": 33, "uom": "mm"} },
    //     doc! { "item": "Air pods", "qty": 23, "status": "B", "size": { "h": 10, "w": 33, "uom": "mm"} },
    //     doc! { "item": "Wireless Charger", "qty": 23, "status": "B", "tags": ["Blue", "Green"] },
    // ];
    // let docs = vec![
    //     Product { item: "Camera", qty: 23, status: "B", size: { h: 10, w: 33, uom: "mm"} },
    //     Product { item: "Air pods", qty: 23, status: "B", size: { h: 10, w: 33, uom: "mm"} },
    //     Product { item: "Wireless Charger", qty: 23, status: "B", tags: ["Blue", "Green"] },
    // ];


    // let products : [Product; 3]  = [
    //     Product { item: "Camera".to_string(), qty: 23},
    //     Product { item: "Air pods".to_string(), qty: 23},
    //     Product { item: "Wireless Charger".to_string(), qty: 23}
    // ];

    
    // println!("\n--------- Inserting multiple records --------");
    // let result = collection.insert_many(docs, None).await?;
    // let result = collection.insert_many(products, None).await?;
    // println!("Newly inserted records : {:?}", result);

    let filter = doc! { "item": "Tablet" };
    let filtered_record = collection.find_one(filter, None).await?;
    println!("Filtered Record : {:?}", filtered_record);
    
    Ok(())
}   

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    item: String,
    qty: u16,
}