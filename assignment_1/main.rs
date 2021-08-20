use std::thread;
use async_std::task;
use std::time::Duration;
use std::time::Instant;
use futures::executor::block_on;

fn main() {
    // aSync Code
    println!("aSync Processing");
    browsing_web();
    block_on(do_concurrently());
    // Multi-threads Code
    println!("");
    println!("Multi-Thread Processing");
    message_notification();
}

// aSync Functions
fn browsing_web() {
    println!("browsing web");
}

async fn file_download() {
    println!("started file download");
    // takes 2 mins
    task::sleep(Duration::from_millis(2000)).await;
    println!("file download complete");
}

fn file_open() {
    println!("opening downloaded file");
    // takes 0.5 seconds
    thread::sleep(Duration::from_millis(0500));
}

async fn do_concurrently() {
    let future = file_download();
    future.await;
    file_open();
    println!("Browser closed")
}

// Multi-threading functions
fn message_content() {
    println!("Show message text");
}

fn download_media() {
    println!("Downloading attachment");
    thread::sleep(Duration::from_millis(5000));
    println!("Downloading complete");
}

fn message_notification() {
    println!("New Message Received");
    let thread_1 = thread::spawn(|| {
        download_media();
        
    });
    let thread_2 = thread::spawn(|| {
        message_content();
    });
    thread_1.join().expect("Thread 1 Panic Error");
    thread_2.join().expect("Thread 2 Panic Error");
    println!("Message read");
}