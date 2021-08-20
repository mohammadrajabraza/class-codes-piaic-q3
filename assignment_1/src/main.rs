
use std::time::Duration;
use std::time::Instant; 

// multi-threading examples/scenario
use std::thread;

fn download_file_browser_tab1(){
    thread::sleep(Duration::from_secs(11));
    println!("File from Browser Tab1 downloaded.");
}

fn download_file_browser_tab2(){
    thread::sleep(Duration::from_secs(11));
    println!("File from Browser Tab2 downloaded.");
}

fn download_files_on_single_thread()
{
    thread::spawn(|| {
        let start = Instant::now();
        let elapsed = start.elapsed();
        download_file_browser_tab1();
        download_file_browser_tab2();
        println!("Time elapsed {:?}", elapsed);
    });
}

fn download_files_in_parallel(){

    let  start = Instant::now();
    let  elapsed = start.elapsed();

    let thread1 = thread::spawn(|| {
        download_file_browser_tab1();
    });

    let thread2 = thread::spawn(|| {
        download_file_browser_tab2();
    });

    thread1.join().expect("Thread 1 panicked!");
    thread2.join().expect("Thread 2 panicked!");

    println!("Time elapsed {:?}", elapsed);
}


// Async examples/scenario
use async_std::task;
use futures::executor::block_on;

fn take_shower() {
    println!("Taking shower");
    thread::sleep(Duration::from_secs(7));
    println!("Finished taking shower");
}

async fn eat_breakfast() {
    println!("Eating breakfast");
    thread::sleep(Duration::from_secs(4));
    println!("Finished eating breakfast");
}

fn get_dressed() {
    println!("Puton dress");
    thread::sleep(Duration::from_secs(6));
    println!("Finished dressing");
}

async fn read_news() {
    println!("Reading news");
    thread::sleep(Duration::from_secs(4));
    println!("Finished reading news");
}

async fn morning_routine(){
    take_shower(); 
    task::spawn(eat_breakfast()); //async
    let _ = read_news().await;
    get_dressed();
      
}


fn main() {
    println!("Assignment 1");

    println!("1- Multi-Threading Example");
    println!("Multiple files downloading using Single thread...");
    download_files_on_single_thread();
    thread::sleep(Duration::from_secs(25));
    println!("Multiple files downloading using Parallel threads...");
    download_files_in_parallel();
    
    println!("2- Async Example");
    block_on(morning_routine());
    
}