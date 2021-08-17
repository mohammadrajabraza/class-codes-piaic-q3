use futures::executor::block_on;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("Number {} from spawn thread", i);
//             thread::sleep(Duration::from_millis(500));
//         }
//     });
//     for i in 1..5 {
//         println!("Number {} from main thread", i);
//         thread::sleep(Duration::from_millis(500));
//     }
// }
/*
In the above example have a spawn thread in the main thread while execution
both threads execute simultaneously but its not gurantee about the sequence and wait to finish the spawn thread.
To avoid this problem we can bind value of spawn thread to a variable which return a joinHandle
*/
//----------------------------------------------------//
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("number{} from spawned thread", i);
//         }
//     });
//     handle.join().unwrap();
//     for i in 0..5 {
//         println!("number {} from main thread", i);
//     }
// }

//In this case join blocks main until handel is done.
//--------------Async/.await-----------------//

// async fn upload_message(){
//     println!("reply message has been uploaded");//Task 2
// }
// async fn download_message(){
//     println!("message has been downloaded"); // Task 1
//     upload_message().await; //task 2 is interleaved with task1
//     //this async function will not end until reply message upload.
//     println!("End of process") //Task 3
// }
// fn main(){
//     block_on(download_message());
// }
//---------------------------------------------------------------//
// async fn upload_message() {
//     println!("reply message has been uploaded");
//     thread::sleep(Duration::from_secs(1));
// }
// async fn download_message() {
//     println!("message has been downloaded");
//     thread::sleep(Duration::from_secs(1));
// }

// async fn responding_message() {
//     download_message().await;
//     upload_message().await;
// }
// fn main() {
//     block_on(responding_message());
// }
//------------------------------------------------//
struct State {
    count: u8,
}
// async fn task2(state:&mut State) ->u8{
//     state.count += 1;
//     print!("Print second statement");
// 1
// }
async fn task2(state: &Arc<Mutex<State>>) -> u8 {
    if let Ok(mut state) = state.lock() {
        state.count += 1;
    }
    println!("Hello world again");
    thread::sleep(Duration::from_secs(1));
    2
}
// async fn task1(state:&mut State) ->u8{
//     state.count += 1;
//     print!("Print first statement");
// 1
// }
async fn task1(state: &Arc<Mutex<State>>) -> u8 {
    if let Ok(mut state) = state.lock() {
        state.count += 1;
    }
    println!("hello world");
    thread::sleep(Duration::from_millis(500));
    1
}

async fn async_main() {
    let state = State { count: 0 };
    // task1(&mut state).await;
    // task2(&mut state).await;
    let data = Arc::new(Mutex::new(state));
    // In following case it gives error with joinHandel there is possibility to share mut state at the same time.
    // let (result1, result2) = futures::join!(task1(&mut state), task2(&mut state));
    let (result1, result2) = futures::join!(task1(&data), task2(&data));
    println!("Result1 is:{},and result 2 is {}", result1, result2);
    if let Ok(s) = data.lock() {
        println!("State: {}", s.count);
    };
}
fn main() {
    block_on(async_main());

    block_on(async {
        for i in 0..5 {
            println!("Value of i :{}", i);
        }
    });
}
