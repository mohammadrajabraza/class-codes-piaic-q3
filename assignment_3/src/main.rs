use futures::stream::TryStreamExt;
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    category: String,
    price: u32,
    dimension: Dimension,
    stock: u16,
    availibility: bool,
    color: Vec<String>,
    has_offer: bool,
    offer_price: u16,
    delivery_charge: u16
}

#[derive(Debug, Serialize, Deserialize)]
struct Dimension {
    width: f32,
    height: f32,
    length: f32,
    unit: String
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    dotenv().ok();

    //Connectiong string for the database through environment variable
    let mut client_options = ClientOptions::parse(env::var("DB_URL").unwrap()).await?;
    client_options.app_name = Some("Rust-And-Mongodb-Store-App".to_string());
    let client = Client::with_options(client_options)?;

    //Get a database
    let db = client.database("store");

    //Show collection in database
    println!("\n--Collections in Database--");
    for collection_names in db.list_collection_names(None).await? {
        println!("{}", collection_names);
    }
    
    // //---Inserting Data---
    // //Inserting one product
    // let collection = db.collection::<Product>("products");

    // let mut colors: Vec<String> = Vec::new();
    // colors = vec!["Black".to_string(), "Grey".to_string(), "White".to_string()];
    // let product_1 = Product { name: "Apple Ipad Pro".to_string(), category: "Tablet".to_string(), price: 125599, 
    // dimension: Dimension {width:2.0, height:6.0, length:4.5, unit:"mm".to_string()}, stock: 16, availibility: true, color: colors,
    // has_offer: false, offer_price:0, delivery_charge:0};

    // let insert_one = collection.insert_one(product_1, None).await?;
    // println!("Newly inserted record : {:?}", insert_one);

    // //Inserting multiple products
    // let collection = db.collection::<Product>("products");
    // let color1 = vec!["Black".to_string()];
    // let color2 = vec!["Black".to_string(), "White".to_string()];
    // let color3 = vec!["White".to_string()];
    // let color4 = vec!["Grey".to_string()];
    // let color5 = vec!["Blue".to_string(), "Black".to_string()];
    // let color6 = vec!["Green".to_string(), "Grey".to_string(), "White".to_string()];
    // let color7 = vec!["Red".to_string(), "Blue".to_string(), "White".to_string()];
    // let color8 = vec!["Black".to_string()];
    // let color9 = vec!["Silk Red".to_string(), "Cosmic Blue".to_string(), "Charcoal Black".to_string()];
    // let color10 = vec!["Black".to_string(), "Maroon".to_string()];
    // let color11 = vec!["Black".to_string(), "Silver".to_string()];
    // let color12 = vec!["White".to_string()];
    // let color13 = vec!["Black".to_string(), "Silver".to_string()];
    // let color14 = vec!["Green".to_string(), "Pink".to_string(), "Blue".to_string(), "Orange".to_string()];
    // let color15 = vec!["Black".to_string()];
    // let color16 = vec!["White".to_string(), "Silver".to_string(), "Grey".to_string()];
    // let color17 = vec!["Black".to_string()];
    // let color18 = vec!["White".to_string()];
    // let color19 = vec!["Black".to_string(), "Gray".to_string()];

    // let products = vec![

    //     Product { name: "HD Television".to_string(), category: "Home Appliances".to_string(), price: 56000, 
    //             dimension: Dimension {width:3.0, height:38.0, length:56.0, unit:"in".to_string()}, stock: 20, availibility: true, color: color1,
    //             has_offer: true, offer_price:55000, delivery_charge:0},

    //     Product { name: "Printer".to_string(), category: "Computer And Accessories".to_string(), price: 12000, 
    //             dimension: Dimension {width:420.0, height:300.0, length:500.5, unit:"mm".to_string()}, stock: 30, availibility: true, color: color2,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "Home Security Camera".to_string(), category: "Cameras And Imaging".to_string(), price: 6500, 
    //             dimension: Dimension {width:50.0, height:100.0, length:50.5, unit:"mm".to_string()}, stock: 15, availibility: true, color: color3,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "INBOOK X1 i3".to_string(), category: "Computer And Accessories".to_string(), price: 79999, 
    //             dimension: Dimension {width:14.0, height:13.0, length:2.5, unit:"in".to_string()}, stock: 20, availibility: true, color: color4,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "Haylou LS02".to_string(), category: "Accessories And Gadgets".to_string(), price: 4999, 
    //             dimension: Dimension {width:2.0, height:1.5, length:4.5, unit:"in".to_string()}, stock: 5, availibility: true, color: color5,
    //             has_offer: true, offer_price:4799, delivery_charge:0},

    //     Product { name: "IMILAB W12".to_string(), category: "Accessories And Gadgets".to_string(), price: 7599, 
    //             dimension: Dimension {width:2.0, height:2.0, length:4.5, unit:"mm".to_string()}, stock: 0, availibility: false, color: color6,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "Gionee Handsfree".to_string(), category: "Accessories And Gadgets".to_string(), price: 399, 
    //             dimension: Dimension {width:0.4, height:0.2, length:3.0, unit:"m".to_string()}, stock: 100, availibility: true, color: color7,
    //             has_offer: false, offer_price:0, delivery_charge:100},

    //     Product { name: "realme Buds Q".to_string(), category: "Accessories And Gadgets".to_string(), price: 5999, 
    //             dimension: Dimension {width:2.0, height:2.0, length:3.5, unit:"in".to_string()}, stock: 15, availibility: true, color: color8,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "Frost Freezer".to_string(), category: "Home Appliances".to_string(), price: 155990, 
    //             dimension: Dimension {width:2.9, height:6.0, length:3.0, unit:"ft".to_string()}, stock: 10, availibility: true, color: color9,
    //             has_offer: false, offer_price:0, delivery_charge:1000},

    //     Product { name: "Candy Air Conditioner".to_string(), category: "Home Appliances".to_string(), price: 68999, 
    //             dimension: Dimension {width:1.0, height:1.5, length:2.5, unit:"ft".to_string()}, stock: 35, availibility: true, color: color10,
    //             has_offer: false, offer_price:0, delivery_charge:500},  

    //     Product { name: "Mi Wireless Power Bank 10000mah".to_string(), category: "Accessories And Gadgets".to_string(), price: 3900, 
    //             dimension: Dimension {width:14.79, height:7.06, length:1.66, unit:"cm".to_string()}, stock: 100, availibility: true, color: color11,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "33W Wall Charger Type-C".to_string(), category: "Accessories And Gadgets".to_string(), price: 2499, 
    //             dimension: Dimension {width:83.7, height:48.0, length:28.0, unit:"mm".to_string()}, stock: 150, availibility: true, color: color12,
    //             has_offer: true, offer_price:2300, delivery_charge:300},

    //     Product { name: "Ciga Machenical Watch".to_string(), category: "Lifestyle".to_string(), price: 25000, 
    //             dimension: Dimension {width:30.0, height:30.0, length:190.5, unit:"mm".to_string()}, stock: 0, availibility: false, color: color13,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "TS Blu-Ray Glasses".to_string(), category: "Lifestyle".to_string(), price: 3900, 
    //             dimension: Dimension {width:150.0, height:60.0, length:110.1, unit:"mm".to_string()}, stock: 50, availibility: true, color: color14,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "Mini Compact Backpack".to_string(), category: "Lifestyle".to_string(), price: 1349, 
    //             dimension: Dimension {width:500.0, height:700.00, length:300.0, unit:"mm".to_string()}, stock: 35, availibility: true, color: color15,
    //             has_offer: false, offer_price:0, delivery_charge:200},

    //     Product { name: "70mai Dash Cam Pro".to_string(), category: "Cameras And Imaging".to_string(), price: 13999, 
    //             dimension: Dimension {width:50.0, height:30.0, length:30.0, unit:"mm".to_string()}, stock: 25, availibility: true, color: color16,
    //             has_offer: false, offer_price:0, delivery_charge:0},

    //     Product { name: "HD 720p Webcam".to_string(), category: "Cameras And Imaging".to_string(), price: 3200, 
    //             dimension: Dimension {width:140.0, height:25.0, length:20.0, unit:"mm".to_string()}, stock: 50, availibility: true, color: color17,
    //             has_offer: false, offer_price:0, delivery_charge:100},

    //     Product { name: "Vacuum Cleaner G10".to_string(), category: "Home Appliances".to_string(), price: 39999, 
    //             dimension: Dimension {width:1286.0, height:214.0, length:256.0, unit:"mm".to_string()}, stock: 3, availibility: true, color: color18,
    //             has_offer: true, offer_price:35000, delivery_charge:300},

    //     Product { name: "Temperature and Humidity Monitor".to_string(), category: "Tablet".to_string(), price: 1500, 
    //             dimension: Dimension {width:100.0, height:50.0, length:25.5, unit:"mm".to_string()}, stock: 250, availibility: true, color: color19,
    //             has_offer: false, offer_price:0, delivery_charge:100},
    // ];

    // let insert_multiple = collection.insert_many(products, None).await?;
    // println!("Newly inserted records : {:#?}", insert_multiple);

    //---Query Products---

    let mut filter;

    //--Comparison Operator--

    // Equal Operator
    filter = doc! { "price": { "$eq": 25000 } };

    // Greater then Operator
    // filter = doc! { "price": { "$gt": 50000 } };

    // Greater then Equal Operator
    // filter = doc! { "price": { "$gte": 79999 } };

    // Less then Operator
    // filter = doc! { "stock": { "$lt": 50 } };

    // Greater then Equal Operator
    // filter = doc! { "stock": { "$lte": 35 } };

    // In Operator
    // filter = doc! { "color": { "$in": ["Orange","Blue"] } };

    // Not Equal Operator
    // filter = doc! { "color": { "$ne": ["White","Silver"] } };

    // Not In Operator
    // filter = doc! { "color": { "$nin": ["Black","White"] } };

    //--Logical Operator--

    //And Operator
    // filter = doc! { "$and": [ {"has_offer": false}, {"availibility": true} ] };

    //Or Operator
    // filter = doc! { "$or": [ {"has_offer": false}, {"availibility": true} ] };

    //--Array Operators--

    // All Operator
    // filter = doc! { "color": {"$all": ["Black", "White"]} };

    // Array Size Operator
    // let filter = doc! { "color": {"$size": 2} };  

    let collection = db.collection::<Product>("products");
    let mut records = collection.find(filter,None).await?;   
     while let Some(product) = records.try_next().await? {
        println!("------------------------------------------");
        println!("{:#?}\n", product);
    }

    //---Update Products---

    // Update Single product

    // let filter = doc! {"name": "Frost Freezer"};
    // let update = doc! {"$set" : {"stock" : 10} };
    // let updated_record = collection.find_one_and_update(filter, update, None).await?;
    // println!("Updated record:");
    // println!("{:#?}", updated_record);

    // Update multiple records

    // let filter = doc! {"category": "Lifestyle"};
    // let update = doc! {"$set" : {"has_offer" : true} };
    // let updated_records = collection.update_many(filter, update, None).await?;
    // println!("Updated records:");
    // println!("{:?}", updated_records);

    //---Delete Products---

    // Delete single record

    // let filter = doc! {"name": "Apple Ipad Pro"};
    // let deleted_record = collection.delete_one(filter, None).await?;
    // println!("Deleted record: {:?}", deleted_record);

    // Delete Multiple record

    // let filter = doc! {"category": "Lifestyle"};
    // let deleted_records = collection.delete_many(filter, None).await?;
    // println!("Deleted records: {:?}", deleted_records);

    Ok(())
}
