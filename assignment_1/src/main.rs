use async_std::task;
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

//Async Functions

//Continues Task
async fn is_music_on() -> bool {
    true
}

//Subtask 1
async fn brew_coffee () {
    println!("Starts Brewing Coffee");
    toast_bread().await;
    task::sleep(Duration::from_millis(3000)).await;
    println!("-End Brewing Coffee-");
}

//Subtask 2
async fn toast_bread () {
    println!("Starts Toasting Bread");
    task::sleep(Duration::from_millis(1000)).await;
    println!("-End Toasting Bread-");
}

//Final Task
async fn serve_breakfast() {

    let  music_status = is_music_on().await;

    if music_status {
        println!("Music is On");
    } else {
        println!("Turn on the Music");
    }
    brew_coffee().await;

    println!("--Breakfast is ready to serve--");
}


fn main() {

    //Multi-thread Example - 1---------------------------------------------------------------

    thread::spawn(|| {
        for i in 1..10 {
            println!("Iteration number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Iteration number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // If we end the program here main thread will exits

    //Async Example - 2----------------------------------------------------------------------

    block_on(serve_breakfast());


}