// Example 1 (Basic Code)
/*
fn _hello() {
    print!("Hello, ");
}

fn _world() {
    println!("_world");
}

fn main() {
    // by default functions run synchronously
    _hello(); //once _hello executed, 
    _world(); //_world start executing
}
*/

// Example 2 (Multi-threaded application)
/*

use std::thread;
use std::time::Duration;
use std::time::Instant; 
fn upload_media() {
    println!("Uploading media start");
    thread::sleep(Duration::from_millis(5000));
    println!("Uploading media ends");
}

fn related_post_details() {
    println!("Poting other details start");
    thread::sleep(Duration::from_millis(4000));
    println!("Posting other details ends");
}

fn add_post(){
    let thread_1 = thread::spawn(|| {
        upload_media();
    });

    let thread_2 = thread::spawn(|| {
        related_post_details();
    });

    thread_1.join().expect("Thread one panicked");
    thread_2.join().expect("Thread two panicked");

}

fn post_acknowledgement() {
    add_post(); //task
    println!("Post added successfully"); //task
}


fn browse_newsfeed(){
    println!("Browsing newsfeed!"); //task
}

fn main() {
    let start = Instant::now();
    post_acknowledgement(); //task can be multi-threaded
    // add_post();
    // upload_media();
    // browse_newsfeed(); //task can be multi-threaded
    let elapsed = start.elapsed();
    println!("Time elapsed {:?}", elapsed);
}
*/

//  Example 3 (Basic Syntax of async functions)
async function
async fn hello_world(){
    println!("test");
}

fn main(){
    hello_world();    
}