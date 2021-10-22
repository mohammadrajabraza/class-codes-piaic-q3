# MongoDB DDL & DML

## Connect to the MongoDB Atlas
```console
mongosh "mongodb+srv://cluster0.isszu.mongodb.net/store" --username xyzabc
Enter password: ********
```

## Create Store
```console
db.createCollection("store", { capped : true, size : 5242880, max : 5000 } )
```

## Insert Collection
```console
Atlas atlas-xos8nh-shard-0 [primary] store> db.products.insertOne({
         "name":"Brown eggs",
         "category":"dairy",
         "weight":"50 gms",
         "dimensions":{"height":10, "width":20, "depth":5, "unit":"dozen"},
         "stock_quantity" : 20,
         "price" : 100,
         "hasOffer" : false,
         "offerPrice" : 0,
         "expiry_date" : 20211004  
     })
    
Atlas atlas-xos8nh-shard-0 [primary] store> db.products.insertMany(
    [{  
        "name":"Sweet LOL stawberry",
        "category":"fruit",
        "weight":"500 gms",
        "dimensions":{"height":20, "width":20, "depth":5, "unit":"kg"},
        "stock_quantity" : 100,
        "price" : 250,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211005  
    },
    {
        "name":"Asparagus",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":50, "width":70, "depth":5, "unit":"dozen"},
        "stock_quantity" : 450,
        "price" : 130,
        "hasOffer" : true,
        "offerPrice" : 99,
        "expiry_date" : 20211006  
    },
    {
        "name":"Green smoothie",
        "category":"dairy",
        "weight":"50 ltr",
        "dimensions":{"height":30, "width":50, "depth":7, "unit":"liters"},
        "stock_quantity" : 2550,
        "price" : 100,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Raw legums",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":100, "width":250, "depth":50, "unit":"kg"},
        "stock_quantity" : 20000,
        "price" : 500,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Pesto with basil",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":70, "width":20, "depth":5, "unit":"kg"},
        "stock_quantity" : 1000,
        "price" : 100,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Hazelnut in black ceramic bowl",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":25, "width":50, "depth":10, "unit":"piece"},
        "stock_quantity" : 250,
        "price" : 1050,
        "hasOffer" : true,
        "offerPrice" : 990,
        "expiry_date" : 20211008 
    },
    {
        "name":"Lemon and salt",
        "category":"fruit",
        "weight":"50 gms",
        "dimensions":{"height":190, "width":230, "depth":7, "unit":"kg"},
        "stock_quantity" : 450,
        "price" : 150,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Homemade bread",
        "category":"bakery",
        "weight":"50 gms",
        "dimensions":{"height":90, "width":70, "depth":9, "unit":"piece"},
        "stock_quantity" : 250,
        "price" : 70,
        "hasOffer" : true,
        "offerPrice" : 60,
        "expiry_date" : 20211003  
    },
    {
        "name":"Legums",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":75, "width":35, "depth":20, "unit":"kg"},
        "stock_quantity" : 250,
        "price" : 1070,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211001  
    },
    {  
        "name":"Fresh tomato",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":10, "width":20, "depth":5, "unit":"dozen"},
        "stock_quantity" : 20,
        "price" : 100,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Blueberry",
        "category":"fruit",
        "weight":"500 gms",
        "dimensions":{"height":20, "width":20, "depth":5, "unit":"kg"},
        "stock_quantity" : 100,
        "price" : 250,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211005  
    },
    {
        "name":"Green beans",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":50, "width":70, "depth":5, "unit":"dozen"},
        "stock_quantity" : 450,
        "price" : 130,
        "hasOffer" : true,
        "offerPrice" : 99,
        "expiry_date" : 20211006  
    },
    {
        "name":"Baked stuffed portabello mushrooms",
        "category":"bakery",
        "weight":"50 ltr",
        "dimensions":{"height":30, "width":50, "depth":7, "unit":"liters"},
        "stock_quantity" : 2550,
        "price" : 100,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Pears juice",
        "category":"bevrages",
        "weight":"50 ml",
        "dimensions":{"height":100, "width":250, "depth":50, "unit":"liters"},
        "stock_quantity" : 20000,
        "price" : 500,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {  
        "name":"Mocha Coffee",
        "category":"bevrages",
        "weight":"50 ml",
        "dimensions":{"height":70, "width":20, "depth":5, "unit":"liters"},
        "stock_quantity" : 1000,
        "price" : 100,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Peach Ice Tea",
        "category":"bevrages",
        "weight":"50 ml",
        "dimensions":{"height":25, "width":50, "depth":10, "unit":"liters"},
        "stock_quantity" : 250,
        "price" : 1050,
        "hasOffer" : true,
        "offerPrice" : 990,
        "expiry_date" : 20211008 
    },
    {
        "name":"Vegan food",
        "category":"vegetable",
        "weight":"50 gms",
        "dimensions":{"height":190, "width":230, "depth":7, "unit":"kg"},
        "stock_quantity" : 450,
        "price" : 150,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211004  
    },
    {
        "name":"Honey",
        "category":"bakery",
        "weight":"50 gms",
        "dimensions":{"height":90, "width":70, "depth":9, "unit":"piece"},
        "stock_quantity" : 250,
        "price" : 70,
        "hasOffer" : true,
        "offerPrice" : 60,
        "expiry_date" : 20211003  
    },
    {
        "name":"Ricotta",
        "category":"dairy",
        "weight":"50 ml",
        "dimensions":{"height":75, "width":35, "depth":20, "unit":"liters"},
        "stock_quantity" : 250,
        "price" : 1070,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211001  
    },
    {
        "name":"Granola",
        "category":"dairy",
        "weight":"50 ml",
        "dimensions":{"height":75, "width":35, "depth":20, "unit":"liters"},
        "stock_quantity" : 250,
        "price" : 1070,
        "hasOffer" : false,
        "offerPrice" : 0,
        "expiry_date" : 20211001  
    }])
```

## Update Collection
```console
Atlas atlas-xos8nh-shard-0 [primary] store> db.products.updateMany( {name:{$eq:"Baked stuffed portabello mushrooms"}}, {$set:{weight:"60 gms"}})
{
  acknowledged: true,
  insertedId: null,
  matchedCount: 1,
  modifiedCount: 1,
  upsertedCount: 0
}
```

## Delete document
```console
Atlas atlas-xos8nh-shard-0 [primary] store> db.products.deleteOne( {name: "Baked stuffed portabello mushrooms"})
{ acknowledged: true, deletedCount: 1 }
```

## Read/Query docuents
```console
Atlas atlas-xos8nh-shard-0 [primary] store> db.products.find( {category: { $eq : "dairy"}}).limit(2)
[
  {
    _id: ObjectId("615b2b5e0ed35048aa164271"),
    name: 'Green smoothie',
    category: 'dairy',
    weight: '50 ltr',
    dimensions: { height: 30, width: 50, depth: 7, unit: 'liters' },
    stock_quantity: 2550,
    price: 100,
    hasOffer: false,
    offerPrice: 0,
    expiry_date: 20211004
  },
  {
    _id: ObjectId("615b2d710ed35048aa164281"),
    name: 'Ricotta',
    category: 'dairy',
    weight: '50 ml',
    dimensions: { height: 75, width: 35, depth: 20, unit: 'liters' },
    stock_quantity: 250,
    price: 1070,
    hasOffer: false,
    offerPrice: 0,
    expiry_date: 20211001
  }
]
```