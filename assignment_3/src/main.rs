use futures::stream::TryStreamExt;
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Making Database connection
    let mut client_options = ClientOptions::parse("mongodb://vidly-user:12345@vidly-shard-00-00.ypxnu.mongodb.net:27017,vidly-shard-00-01.ypxnu.mongodb.net:27017,vidly-shard-00-02.ypxnu.mongodb.net:27017/vidly?ssl=true&replicaSet=atlas-1437c6-shard-0&authSource=admin&retryWrites=true&w=majority").await?;

    // Set the name of the app
    client_options.app_name = Some("Rust Store App".to_string());   

    // Get a handle to the client
    let client = Client::with_options(client_options)?;

    // List the names of the current databases
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // Create a new "store" database
    let db = client.database("store"); 

    // Create a collection "products"
    let collection = db.collection::<Document>("products");

    // -------------------------- Inserting 20 records in products-------------------------------

    // let docs = vec![
    //     doc! {"name": "Tablet", "category": "Electronic Devices", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 10, "width": 50, "depth": 5, "unit": "mm"}, "stock_quantity": 23, "price": 15000, 
    //     "hasOffer": "No", "offerPrice": "Yes", "colors": ["Blue", "Green", "Violet"]},

    //     doc! {"name": "Printer", "category": "Electronic Accessories", "status": "InStock", "weight": 15, 
    //     "dimensions": {"height": 120, "width": 250, "depth": 50, "unit": "mm"}, "stock_quantity": 10, "price": 25000, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["White", "Black"]},

    //     doc! {"name": "Television", "category": "TV & Home Appliances", "status": "InStock", "weight": 20, 
    //     "dimensions": {"height": 100, "width": 200, "depth": 40, "unit": "mm"}, "stock_quantity": 40, "price": 28000, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["Blue", "Black", "Gray"]},

    //     doc! {"name": "Makeup Kit", "category": "Health & Beauty", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 10, "width": 20, "depth": 5, "unit": "mm"}, "stock_quantity": 150, "price": 5000, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Pink", "Red", "Yellow"]},

    //     doc! {"name": "Baby Cart", "category": "Babies & Toys", "status": "InStock", "weight": 10, 
    //     "dimensions": {"height": 140, "width": 200, "depth": 35, "unit": "mm"}, "stock_quantity": 38, "price": 10000, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["White", "Black", "Blue"]},

    //     doc! {"name": "Snacks", "category": "Groceries", "status": "InStock", "weight": 2, 
    //     "dimensions": {"height": 15, "width": 20, "depth": 5, "unit": "mm"}, "stock_quantity": 517, "price": 500, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Green", "Red", "Purple", "Violet"]},

    //     doc! {"name": "Watch", "category": "Watches & Bags", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 6, "width": 5, "depth": 3, "unit": "mm"}, "stock_quantity": 410, "price": 1000, 
    //     "hasOffer": "No", "offerPrice": "Yes", "colors": ["White", "Silver", "Black"]},

    //     doc! {"name": "Rackets", "category": "Sports", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 10, "width": 7, "depth": 5, "unit": "mm"}, "stock_quantity": 306, "price": 1000, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Pink", "Red"]},

    //     doc! {"name": "Honda Civic", "category": "Automotive & Bikes", "status": "InStock", "weight": 200, 
    //     "dimensions": {"height": 250, "width": 600, "depth": 250, "unit": "mm"}, "stock_quantity": 17, "price": 200000, 
    //     "hasOffer": "Yes", "offerPrice": "Yes", "colors": ["White", "Black", "Gray"]},

    //     doc! {"name": "Laptop", "category": "Electronic Devices", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 5, "width": 50, "depth": 5, "unit": "mm"}, "stock_quantity": 138, "price": 40000, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["Silver", "White", "Black"]},

    //     //     10 till here --------------------------------------------------------

    //     doc! {"name": "Drone Camera", "category": "Electronic Devices", "status": "InStock", "weight": 5, 
    //     "dimensions": {"height": 10, "width": 20, "depth": 5, "unit": "mm"}, "stock_quantity": 26, "price": 10000, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Black", "Blue"]},

    //     doc! {"name": "Smart Watch", "category": "Wearable", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 12, "width": 13, "depth": 7, "unit": "mm"}, "stock_quantity": 59, "price": 2000, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["Blue", "Black", "Silver"]},

    //     doc! {"name": "Emotion", "category": "Health & Beauty", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 20, "width": 25, "depth": 3, "unit": "mm"}, "stock_quantity": 1158, "price": 1200, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Gray", "Red", "Yellow", "Purple"]},

    //     doc! {"name": "Car", "category": "Babies & Toys", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 40, "width": 20, "depth": 15, "unit": "mm"}, "stock_quantity": 643, "price": 1500, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Black", "Blue", "Yellow"]},

    //     doc! {"name": "Chocolates", "category": "Groceries", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 19, "width": 7, "depth": 2, "unit": "mm"}, "stock_quantity": 2067, "price": 100, 
    //     "hasOffer": "No", "offerPrice": "No", "colors": ["White", "Green", "Red"]},

    //     doc! {"name": "Pouch", "category": "Stationary", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 26, "width": 150, "depth": 8, "unit": "mm"}, "stock_quantity": 1407, "price": 500, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["Blue", "Orange"]},

    //     doc! {"name": "Bat", "category": "Sports", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 150, "width": 27, "depth": 5, "unit": "mm"}, "stock_quantity": 1306, "price": 1500, 
    //     "hasOffer": "No", "offerPrice": "Yes", "colors": ["Skin", "Black", "Violet"]},

    //     doc! {"name": "CG-125", "category": "Automotive & Bikes", "status": "InStock", "weight": 80, 
    //     "dimensions": {"height": 150, "width": 300, "depth": 100, "unit": "mm"}, "stock_quantity": 93, "price": 160000, 
    //     "hasOffer": "Yes", "offerPrice": "Yes", "colors": ["Red", "Black", "Blue"]},

    //     doc! {"name": "Shoes", "category": "Home & Lifestyle", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 27, "width": 35, "depth": 17, "unit": "mm"}, "stock_quantity": 3458, "price": 4000, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["Brown", "White", "Black", "Gray", "Skin"]},

    //     doc! {"name": "Headphones", "category": "Electronic Accessories", "status": "InStock", "weight": 1, 
    //     "dimensions": {"height": 12, "width": 16, "depth": 9, "unit": "mm"}, "stock_quantity": 432, "price": 1800, 
    //     "hasOffer": "Yes", "offerPrice": "No", "colors": ["Red", "White", "Black"]},
    // ];
    
    // let records = collection.insert_many(docs, None).await?;
    // println!("Inserted records: {:?}", records);

    // ---------------------------------- Applying query operators to find records -----------------------------
    // Comparison operators
    // let filter = doc! { "price": { "$eq": 1800 }  };                         // $eq operator
    // let filter = doc! { "price": { "$gt": 100000 }  };                       // $gt operator
    // let filter = doc! { "price": { "$gte": 10000 }  };                      // $gte operator
    // let filter = doc! { "price": { "$in": [1800, 5000, 10000] }  };         // $in operator
    // let filter = doc! { "price": { "$lt": 1000 }  };                           // $lt operator
    // let filter = doc! { "price": { "$lte": 5000 }  };                      // $lte operator
    // let filter = doc! { "weight": { "$ne": 1 }  };                      // $ne operator
    // let filter = doc! { "colors": { "$nin": ["Black","Blue","Yellow"] }  };  // $nin operator

    // Logical operators
    // let filter = doc! { "$and": [ {"hasOffer": "Yes"}, {"status": "NotInStock"} ] };             // $and operator
    // let filter = doc! { "$or": [ {"hasOffer": "Yes"}, {"status": "NotInStock"} ] };             // $or operator
    // let filter = doc! { "dimensions.depth":  {"$not": {"$lt": 100}} };                          // $not operator
    // let filter = doc! { "$nor": [ {"hasOffer": "No"}, {"price": {"$lt": 5000}} ] };         // $nor operator

    // Element operators
    // let filter = doc! { "colors": {"$exists": true, "$nin": ["Red", "Blue", "White"]} };     // $exists operator
    // let filter = doc! { "weight": {"$type": "double"} };             // $type operator

    // Array operators
    // let filter = doc! { "colors": {"$all": ["Red", "Blue", "Black"]} };     // $all operator
    // let filter = doc! { "colors": {"$elemMatch": {"$eq": "Violet"}} };             // $elemMatch operator
    // let filter = doc! { "colors": {"$size": 2} };             // $size operator


    // let mut records = collection.find(filter,None).await?;

    // println!("Extracted records are:");

    // while let Some(product) = records.try_next().await? {
    //     // println!("Name: {}", product.get_str("name").unwrap());    // display name of the product
    //     println!("-------------------------------------------------------------");
    //     println!("{}", product);
    // }

    // ------------------------------------- Updating records ----------------------------------

    // Replacing single record
    // let filter = doc! {"name": "Pouch"};

    // let replacement = doc! {"name": "Geometry Box", "category": "Stationary", "status": "InStock", "weight": 1, 
    // "dimensions": {"height": 23, "width": 123, "depth": 4, "unit": "mm"}, "stock_quantity": 4322, "price": 350, 
    // "hasOffer": "Yes", "offerPrice": "No", "colors": ["Blue", "Orange", "Gray"]};

    // let replaced_record = collection.find_one_and_replace(filter, replacement, None).await?;
    // println!("Replaced record:");
    // println!("{:?}", replaced_record);

    // Updating single record
    // let filter = doc! {"name": "Drone Camera"};
    // let update = doc! {"$set" : {"stock_quantity" : 30} };
    // let updated_record = collection.find_one_and_update(filter, update, None).await?;
    // println!("Updated record:");
    // println!("{:?}", updated_record);

    // Updating multiple records
    // let filter = doc! {"category": "Electronic Accessories"};
    // let update = doc! {"$set" : {"hasOffer" : "No", "offerPrice": "Yes"} };
    // let updated_records = collection.update_many(filter, update, None).await?;
    // println!("Updated records:");
    // println!("{:?}", updated_records);

    // -------------------------- Inserting record in products for delete operation -------------------------------

    // let document = doc! {"name": "Washing machine", "category": "TV & Home Appliances", "status": "InStock", "weight": 50, 
    //     "dimensions": {"height": 270, "width": 250, "depth": 120, "unit": "mm"}, "stock_quantity": 11, "price": 17000, 
    //     "hasOffer": "Yes", "offerPrice": "Yes", "colors": ["Gray", "Green", "White"]};

    // let record = collection.insert_one(document, None).await?;
    // println!("Inserted record: {:?}", record);

    // ----------------------------- Deleting single record ------------------------------
    // let filter = doc! {"name": "Washing machine"};
    // let deleted_record = collection.delete_one(filter, None).await?;
    // println!("Deleted record: {:?}", deleted_record);


     // ----------------------------- Deleting multiple records ------------------------------
    //  let filter = doc! {"name": "Washing machine"};
    //  let deleted_records = collection.delete_many(filter, None).await?;
    //  println!("Deleted records: {:?}", deleted_records);
    
    Ok(())
}
