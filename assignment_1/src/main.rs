use async_std::task;
use std::time::{Duration,Instant};
use futures::executor::block_on;
use std::thread;
//Async cases
fn clean_room() {
    
    println!("Started cleaning room");    // will 30 minutes
    thread::sleep(Duration::from_millis(3000));
    println!("Finished cleaning room");
}


fn make_green_tea() {
    
    println!("Started making green tea");   // will 10 minutes
    thread::sleep(Duration::from_millis(1000));
    println!("Finished making green tea");
}
    
async fn listening_song() {
    println!("Listening song");
    task::sleep(Duration::from_millis(10000)).await;
    println!("Song is heard");
}
    
async fn wash_dishes(){
    
    println!("Started washing dishes"); //twill take 30 min
    task::sleep(Duration::from_millis(3500)).await; 
    println!("Finished washing dishes");
}
   
async fn do_work_efficiently() {

    task::spawn(listening_song()); 
    
    clean_room();   
    
    let result = wash_dishes().await;  
    make_green_tea();
    
}

//Multi-threading cases
fn main() {
    println!("multi-threading starts here!");

    let start = Instant::now();
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(5000));
        }
    });

   let handle2 = thread::spawn(||{  for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(5000));
    }});
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("Time elapsed is {:#?}",start.elapsed());

    println!("multi-threading ends here!");
    println!("Async starts here!");

    let start2 = Instant::now();
    block_on(do_work_efficiently());

    println!("Time taken : {:?}", start2.elapsed());

    println!("Async ends here!");

}
