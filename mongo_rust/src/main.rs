use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let mut client_options = ClientOptions::parse(env::var("DB_URL").unwrap()).await?;
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
    

    // uncomment below lines to replace an existing record completey with a new one
    // println!("\n--------- Replacing single record --------");
    // let filter = doc! { "item": "Camera" };
    // let replacement = doc! { "blood_group": 23};
    // let replaced_records = collection.find_one_and_replace(filter, replacement, None).await?;
    // println!("Replaced records : {:?}", replaced_records);
    

    // println!("\n--------- Updating single record --------");
    // let filter = doc! { "item": "Camera" };
    // let update = doc! { "$set" : { "qty" : 123 } };
    // let updated_record = collection.find_one_and_update(filter, update, None).await?;
    // println!("Updated records : {:?}", updated_record);

    // println!("\n--------- Updating multiple records --------");
    // let filter = doc! { "item": "Camera" };
    // let update = doc! { "$set" : { "qty" : 333 } };
    // let updated_record = collection.update_many(filter, update, None).await?;
    // println!("Updated records : {:?}", updated_record);
    
    // println!("\n--------- Deleting single record --------");
    // let filter = doc! { "item": "Tablet" };
    // let deleted_record = collection.delete_one(filter, None).await?;
    // println!("Deleted records : {:?}", deleted_record);

    println!("\n--------- Deleting multiple records --------");
    let filter = doc! { "item": "Camera" };
    let deleted_records = collection.delete_many(filter, None).await?;
    println!("Deleted records : {:?}", deleted_records);
    

    // let filter = doc! { "item": "Tablet" };
    // let filtered_record = collection.find_one(filter, None).await?;
    // println!("Filtered Record : {:?}", filtered_record);
    
    Ok(())
}   

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    item: String,
    qty: u16,
}