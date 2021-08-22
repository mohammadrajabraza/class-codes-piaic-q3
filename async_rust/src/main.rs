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

    // let thread_2 = thread::spawn(|| {
        related_post_details();
    // });

    thread_1.join().expect("Thread one panicked");
    // thread_2.join().expect("Thread two panicked");

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
/*
use futures::executor::block_on;
async fn hello_world(){
    println!("test");
}

fn main(){
    block_on(hello_world());    
}
*/


//  Example 4 (Complex async rust example)
use async_std::task;
use std::time::Duration;
use std::time::Instant;
use futures::executor::block_on;
use std::thread;


fn clean_room() {
    // takes 20 minutes
    println!("Started cleaning room");
    thread::sleep(Duration::from_millis(3000));
    println!("Finished cleaning room");
}

// Async -> return future
fn make_tea() {
    // takes 20 minutes
    println!("Started making tea");
    thread::sleep(Duration::from_millis(1000));
    println!("Finished making tea");
}
    
async fn listening_song() {
    println!("Listening song");
    task::sleep(Duration::from_millis(10000)).await;
    println!("Success/Failure");
}
    
async fn wash_dishes(){
    // takes 20 minutes
    println!("Started washing dishes");
    task::sleep(Duration::from_millis(3500)).await; //Pani khatam ho gaya
    println!("Finished washing dishes");
}
   
async fn do_work_efficiently() {

    task::spawn(listening_song()); //async
    
    clean_room();   // 2sec
    
    let result = wash_dishes().await;  //2 sec
    make_tea();
    // let future = make_tea();
    // block_on(future);   // 2sec
    // future.await;// waiting ........
    // task::spawn(make_tea()); // not waiting
    
}

fn main() {
    let start = Instant::now();//00:02
    block_on(do_work_efficiently());


    let elapsed = start.elapsed(); // 00:56 
    println!("Time taken : {:?}", elapsed);
}
