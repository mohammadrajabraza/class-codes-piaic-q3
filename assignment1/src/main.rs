
// IMPELEMNTED SENARIO:
// ------------------- 

/* In  MS WORD app we have three tasks: 
            1. Input from user through keyboard 
            2. Save the file 
            3. Print the file

    If we think in sequential way (sync. Prog.) of execution, it will take 6sec. 
    But by using multithreading in concurrency way (parallel exec. of three tasks), 
    has taken 3.00096509sec.
    By using Async/.await these task can be completed in 5.8027ms.
*/



// ----------------------------------------------------------------------------------------------------
//                                          * MULTI-THREADING WAY  *
// ----------------------------------------------------------------------------------------------------

// use async_std::task;
// use std::thread;
// use std::time::Duration;
// use std::time::Instant;
// use futures::executor::block_on;

// fn keyboard_ip() {
//     println!("Keyboard input starts ...");
//     thread::sleep(Duration::from_millis(3000));
//     println!("Keyboard input stops!");    
// }

// fn save_file(){
//     println!("Start saving the file ...");
//     thread::sleep(Duration::from_millis(2000));
//     println!("File Saved !");
// }

// fn print_func(){
//     println!("Print func. starts ...");
//     thread::sleep(Duration::from_millis(1000));
//     println!("Print func. ends!");
// }

// fn msword_file_funcs(){
//     let thread_1 = std::thread::spawn(|| {
//         keyboard_ip();
//     });
//     let thread_2 = std::thread::spawn(||{
//         save_file();
//     });
//     let thread_3 = std::thread::spawn(||{
//         print_func();
//     });
//     thread_1.join().expect("Thread one panicked");
//     thread_2.join().expect("Thread two panicked");
//     thread_3.join().expect("Thread three panicked");
// }

// fn main() {
//     println!("MS WORD file open...");
//     let strat = Instant::now(); // Timer Start
//     msword_file_funcs();
//     let elapsed = strat.elapsed(); // Timer Stops
//     println!("TIME TAKEN BY PROGRAM: {:?}", elapsed);
// }


// ----------------------------------------------------------------------------------------------------
//                                          * ASYNC. SINGLE-THREAD WAY  *
// ----------------------------------------------------------------------------------------------------

use async_std::task;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use futures::executor::block_on;

async fn keyboard_ip() {
    println!("Keyboard input starts ...");
    task::sleep(Duration::from_millis(3000));
    println!("Keyboard input stops!");    
}

async fn save_file(){
    println!("Start saving the file ...");
    task::sleep(Duration::from_millis(2000));
    println!("File Saved !");
}

async fn print_func(){
    println!("Print func. starts ...");
    task::sleep(Duration::from_millis(1000));
    println!("Print func. ends!");
}

async fn msword_file_funcs(){
    let future_1 = keyboard_ip().await;
    let future_2 = save_file().await;
    let future_3 = print_func().await;
}

fn main() {
    println!("---- MS WORD file open ----");
    let strat = Instant::now(); // Timer Start
    block_on(msword_file_funcs());
    let elapsed = strat.elapsed(); // Timer Stops
    println!("TIME TAKEN BY PROGRAM: {:?}", elapsed);
}
