// -------------------------------------------------------------------
// ----------------------  Simple Get Request-------------------------
// -------------------------------------------------------------------
/*
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{

    let res = reqwest::get("https://www.rust-lang.org")
    .await?;

    let body = res
    .text()
    .await?;

    println!("body = {:?}", body);
    
    Ok(())
}
*/

// -------------------------------------------------------------------
// ----------------------  Simple Get Request-------------------------
// -------------------------------------------------------------------
/*
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Ip {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    
    let ip = reqwest::get("http://httpbin.org/ip")
    .await?.text().await?;
    // .json::<Ip>()
    // .await?;

    println!("ip: {:#?}", ip);
    Ok(())
}
*/
// -------------------------------------------------------------------
// ----------------------  Simple Post Request-------------------------
// -------------------------------------------------------------------
/*
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{

    let client = reqwest::Client::new();

    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;

    println!("Status code for response against request : {}",res.status());
    
    Ok(())
}
*/

// -------------------------------------------------------------------
// ----------------- Simple Post Request For Forms -------------------
// -------------------------------------------------------------------

/*
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    
    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await?;

    println!("Status code for response against request : {}",res.status());
    println!("HTTP version of response against request : {:?}",res.version());
    println!("URL for response against request : {}",res.url());
    println!("Length of content received in response : {:?}",res.content_length());
    println!("Text of response : {:?}",res.text().await);
    
    Ok(())
}
*/

// -------------------------------------------------------------------
// ----------------- Simple Post Request For JSON  -------------------
// -------------------------------------------------------------------
/*
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", echo_json);
    // Object(
    //     {
    //         "body": String(
    //             "https://docs.rs/reqwest"
    //         ),
    //         "id": Number(
    //             101
    //         ),
    //         "title": String(
    //             "Reqwest.rs"
    //         ),
    //         "userId": Number(
    //             1
    //         )
    //     }
    // )
    Ok(())
}
*/
// -------------------------------------------------------------------
// --------------------- Blocking Get Request  -----------------------
// -------------------------------------------------------------------
// reqwest needed

// https://github.com/seanmonstar/reqwest
// `cargo run --example blocking --features=blocking


// fn main() -> Result<(), reqwest::Error>{

//     let body = reqwest::blocking::get("https://www.rust-lang.org")?
//         .text()?;
//     println!("body = {:?}", body);

//     Ok(())
// }

/*
fn main() -> Result<(), reqwest::Error> { 

    println!("GET https://www.rust-lang.org");

    let mut res = reqwest::blocking::get("https://www.rust-lang.org/")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}

*/


/*
fn main() -> Result<(), reqwest::Error> { 

    println!("GET http://localhost::3000");

    let mut res = reqwest::blocking::get("http://localhost:3000/fetchdata")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}
*/

// -------------------------------------------------------------------
// ----------------------  Blocking Post Request-------------------------
// -------------------------------------------------------------------

fn main() -> Result<(), reqwest::Error>{
    
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send()?;

    println!("Status code for response against request : {}",res.status());
    println!("HTTP version of response against request : {:?}",res.version());
    println!("URL for response against request : {}",res.url());
    println!("Length of content received in response : {:?}",res.content_length());
    println!("Text of response : {:?}",res.text());
    
    Ok(())
}



/*
fn main() -> Result<(), reqwest::Error>{
    
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:3000/echo/uppercase")
    .body("Hello World")
    .send()?;

    println!("Status code for response against request : {}",res.status());
    println!("HTTP version of response against request : {:?}",res.version());
    println!("URL for response against request : {}",res.url());
    println!("Length of content received in response : {:?}",res.content_length());
    println!("Text of response : {:?}",res.text());
    
    Ok(())
}*/

