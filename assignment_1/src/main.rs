/*  Assignment_1 Q3 IoT Batch # 35
Eliza Batool - PIAIC-167010 
Multithreading vs Async
*/

// multi-threading example/scenarion

use std::thread;
use std::time::Duration;
use std::time::Instant;
use futures::executor::block_on;
use async_std::task;


fn cook_egg(){
    println!("Egg is being cooked");
    thread::sleep(Duration::from_millis(1000));
    println!("Egg is ready to serve");
}
fn make_tea(){
    println!("Tea is about to boil");
    thread::sleep(Duration::from_millis(1000));
    println!("Tea is ready, Please turn off the stove");
}
fn make_breakfast(){
    let thread_1 = thread::spawn(||{
        cook_egg();        
    });
    let thread_2 = thread::spawn(||{
        make_tea();
    });
    thread_1.join().expect("Thread one panicked");
    thread_2.join().expect("Thread two panicked");
}

// Async example/scenarion

async fn async_cook_egg(){
    println!("Egg is being cooked");
    thread::sleep(Duration::from_millis(1000));
    println!("Egg is ready to serve");
}

async fn async_make_breakfast(){
	task::spawn(async_cook_egg());
    make_tea(); 
}

fn main(){
	println!("Assignment 1 by PIAIC-167010");
	let start1 = Instant::now();
	println!(" Lets start making breakfast without multiple threads \n");
	cook_egg();
	make_tea();
	println!("\n Breakfast is ready, lets eat \n");
	let elapsed1 = start1.elapsed();
    println!("Time elapsed {:?}\n",elapsed1);
	let start2 = Instant::now();
	println!(" Lets start making breakfast with multiple threads \n");
    make_breakfast();
	println!("\n Breakfast is ready, lets eat \n");
    let elapsed2 = start2.elapsed();
    println!("Time elapsed {:?}\n",elapsed2);
	println!(" Lets start making breakfast asynchronously \n");
    let start3 = Instant::now();
    block_on(async_make_breakfast());
	println!("\n Breakfast is ready, lets eat \n");
    let elapsed3 = start3.elapsed();
    println!("Time elapsed {:?}",elapsed3);
} 