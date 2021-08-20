// ----------------------------------- multi-threading example --------------------------------

// use std::time::{Duration, Instant};
// use std::thread;

// fn main() {
//     let start = Instant::now();
//     multi_threaded_function();
//     let end = start.elapsed();
//     println!("Calculated time: {:?}", end);
// }

// fn multi_threaded_function() {
//     let thread_1 = thread::spawn(|| {
//        downloading_file(); 
//     });

//     let thread_2 = thread::spawn(|| {
//         coding(); 
//     });

//     thread_1.join().expect("Unable to download file!");
//     thread_2.join().expect("Unable to do coding!");
// }

// fn downloading_file() {
//     println!("Downloading start...");
//     thread::sleep(Duration::from_millis(5000));
//     println!("Download complete!");
// }

// fn coding() {
//     println!("Coding start...");
//     thread::sleep(Duration::from_millis(2000));
//     println!("Coding end!");
// }

// --------------------------------- async example -----------------------------------

use async_std::task;
use futures::executor::block_on;
use std::time::Duration;
use std::time::Instant;
use std::thread;


fn main() {
    let start = Instant::now();
    block_on(async_functions());
    let elapsed = start.elapsed();
    println!("Calculated time is: {:?}", elapsed);
}

async fn async_functions() {
    task::spawn(listen_songs());
    running().await;
    push_ups();

}

async fn running() {
    println!("Running start");
    task::sleep(Duration::from_millis(2000)).await;
    println!("Running end");
}

async fn listen_songs() {
    println!("Start music");
    task::sleep(Duration::from_millis(5000)).await;
}

fn push_ups() {
    println!("Push ups start");
    thread::sleep(Duration::from_millis(2000));
    println!("Push ups end");
}
