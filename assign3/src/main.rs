use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    
    // database connection store in .env file
    dotenv().ok();

    // println!("{:?}",env::var("DB_CONN").unwrap());
    // let mut client_options = ClientOptions::parse(env::var("DB_CONN").unwrap()).await?;
    let mut client_options = ClientOptions::parse(env::var("DB_CONN").unwrap()).await?;
    client_options.app_name = Some("Rust-Mongo-Connectivity".to_string());
    let client = Client::with_options(client_options)?;

    let db = client.database("store");
    let collection = db.collection::<Product>("products");
    // let collection = db.collection("products");
    
    
    let docs = vec![
      // Product#1
        doc! { "name": "Samsung S2", "category": "Mobile", "manufacturer": "Samsung", "price": 23, "qty": 8, "weight": 1.2, "height": 1.4,"length": 0.8, "width":"0.9","sku":"ABC12","mfgpartno":"ADAF31" },

      // Product#2
        doc! { "name": "Samsung TAB 5", "manufacturer": "Samsung", "category": "Tablet", "price": 42, "qty": 4, "weight": 0.2, "height": 0.4,"length": 1.8, "width":"2.9","sku":"DAE1321","mfgpartno":"S141F" },
      // Product#3
        doc! { "name": "Zen", "category": "Notebook", "manufacturer": "Asus", "price": 142, "qty": 3, "weight": 0.7, "height": 1.4,"length": 2.8, "width":3.9,"sku":"ZNE1321","mfgpartno":"ZN131" },
      // Product#4
        doc! { "name": "Zenbook 8", "category": "Notebook", "manufacturer": "Asus", "price": 153, "qty": 2, "weight": 0.7, "height": 1.4,"length": 3.8, "width":1.9,"sku":"ZEN1321","mfgpartno":"ZENN121" },
      // Product#5
        doc! { "name": "Zen Phone 12", "category": "Mobile", "manufacturer": "Asus", "price": 42, "qty": 5, "weight": 0.7, "height": 1.4,"length": 2.8, "width":3.9,"sku":"ZEN1321","mfgpartno":"ZEN41F" },
      // Product#6
        doc! { "name": "Zen Watch 3", "category": "Smart Watch", "manufacturer": "Asus", "price": 34, "qty": 4, "weight": 0.7, "height": 1.4,"length": 2.8, "width":3.9,"sku":"AEE1321","mfgpartno":"S3D41F" },
      // Product#7
        doc! { "name": "Huawei Chromebook", "category": "Notebook", "manufacturer": "Huawei", "price": 152, "qty": 3, "weight": 2.5, "height": 2.4,"length": 1.8, "width":2.9,"sku":"HUA1321","mfgpartno":"HWE41F" },

      // Product#8
        doc! { "name": "Huawei P8", "category": "Mobile", "manufacturer": "Huawei", "price": 102, "qty": 1, "weight": 0.7, "height": 0.4,"length": 1.8, "width":1.9,"sku":"HUA1321","mfgpartno":"P841F" },

      // Product#9
        doc! { "name": "Huawei Book", "category": "Tablet", "manufacturer": "Huawei", "price": 52, "qty": 2, "weight": 0.7, "height": 1.4,"length": 2.8, "width":3.9,"sku":"HUA521","mfgpartno":"HWP1213" },

      // Product#10
        doc! { "name": "Huawei Watch", "category": "Smart Watch", "manufacturer": "Huawei", "price": 45, "qty": 2, "weight": 0.4, "height": 1.2,"length": 1.8, "width":2.9,"sku":"HUA1321","mfgpartno":"HSWD41F" },

      // Product#11
        doc! { "name": "Samsung Titan 5", "category": "Smart Watch", "manufacturer": "Samsung", "price": 35, "qty": 2, "weight": 0.2, "height": 1.2,"length": 1.2, "width":1.9,"sku":"STT1321","mfgpartno":"SAMTWD41F" },

    // Product#12
        doc! { "name": "Samsung Chromebook", "category": "Notebook", "manufacturer": "Samsung", "price": 135, "qty": 3, "weight": 1.2, "height": 2.2,"length": 3.2, "width":3.9,"sku":"SCH1321","mfgpartno":"SCSWD41F" },

    ];
    // Insert products
    let result = collection.insert_many(docs, None).await?;
    
  // Make a query
    let filter = doc! { "category": "Tablet" };
    let filtered_record = collection.find_one(filter, None).await?;
    println!("Filtered Tablet : {:?}", filtered_record);

// Crud operation
// Creation
    println!("\n--------- Create record --------");

      let doc = Product { 
        name: "Microsoft Office".to_string(), 
        category: "Software".to_string(),
        manufacturer: "Microsoft".to_string(),
        price: 45,
        qty : 23,
        weight: 0.4,
        height: 0.1,
        length: 0.13,
        width: 0.11,
        sku: "MS120".to_string(),
        mfgpartno: "MS1AS1".to_string()
    };
    let record = collection.insert_one(doc, None).await?;
    println!("Newly inserted record : {:?}", record);

  // READ
    println!("\n--------- Read record --------");
    let query = doc! {"manufacturer": "Microsoft"};
    let result = collection.find_one(query, None).await?;
    println!("Find the record : {:?}", result);
  
  // Update 
    println!("\n--------- Update record --------");
    let filter = doc! { "manufacturer": "Microsoft" };
    let update = doc! { "$set" : { "name" : "Microsoft Office Suite" } };
    let updated_record = collection.find_one_and_update(filter, update, None).await?;
    println!("Record Updated : {:?}", updated_record);
 
  // Delete 
 println!("\n--------- Deleting single record --------");
    let filter = doc! { "name": "Microsoft Office Suite" };
    let deleted_record = collection.delete_one(filter, None).await?;
    println!("Record Deleted : {:?}", deleted_record);


    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    category: String,
    manufacturer: String,
    price: u16,
    qty: u8,
    weight: f32,
    height: f32,
    length: f32,
    width: f32,
    sku: String,
    mfgpartno: String
}
