/*  Assignment_1 Q3 IoT Batch # 35
Saim Raza - PIAIC147294 

Multithreading vs Async

*/


//  Multi-threading

// Example_1    // no multi-threading

/*
use std::thread;
use std::time::Duration;
use std::time::Instant;


fn work(){
    println!("started working on schedule task");
    thread::sleep(Duration::from_millis(1000));
    println!("stop working on schedule task");;
}

fn music() {
     println!("start listening to music");
    thread::sleep(Duration::from_millis(1000));
    println!("stop listening to music")
}

fn main() {
    let start = Instant::now();
    work();
    music();
    let elapsed = start.elapsed();
    println!("Time elapsed {:?}",elapsed);

} 
*/

// Example_2    multi-threading implemented

/*
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn work(){
    println!("started working on schedule task");
    thread::sleep(Duration::from_millis(1000));
    println!("stop working on schedule task");
}

fn music(){
    println!("start listening to music");
    thread::sleep(Duration::from_millis(1000));
    println!("stop listening to music");
}

fn both_work(){
    let thread_1 = thread::spawn(||{
        work();        
    });

    let thread_2 = thread::spawn(||{
        music();
    });

    thread_1.join().expect("Thread one panicked");
    thread_2.join().expect("Thread two panicked");
}

fn main(){
    let start = Instant::now();
    both_work();
    let elapsed = start.elapsed();
    println!("Time elapsed {:?}",elapsed);
}
*/


// Async //

// Example 3 

/*
async fn work(){
    println!("hello world");
}

fn main(){
    work();
}
*/

// Example 4
/*
use futures::executor::block_on;

async fn work(){
    println!("hello world");
}

fn main(){
    block_on(work());
}

*/

// Example 5 

use std::thread;
use std::time::Duration;
use std::time::Instant;
use futures::executor::block_on;
use async_std::task;



fn clean_car() {
    // takes 20 minutes
    println!("Started cleaning a car");
    thread::sleep(Duration::from_millis(2100));
    println!("Finished cleaning a car");
}

async fn listen_music(){
// takes 20 mins
    println!("started listening music");
    task::sleep(Duration::from_millis(2000)).await;
    println!("stop listening music");
}

fn packing(){
// take 20 mins
    println!("started packing");
    thread::sleep(Duration::from_millis(2000));
    println!("stop packing");
}

async fn do_work_efficiently(){

    task::spawn(listen_music());
    clean_car(); 
}



fn main(){
    let start = Instant::now();

    packing();
    block_on(do_work_efficiently());
    

    let elapsed = start.elapsed();
    println!("Time elapsed {:?}",elapsed);
}