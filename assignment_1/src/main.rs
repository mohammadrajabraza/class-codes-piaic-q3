use futures::executor::block_on;
use rand::Rng;
use std::thread;
use std::time::Instant;
use threadpool::ThreadPool;
extern crate threadpool;
use futures::future::join_all;


#[tokio::main] 
async fn main() {
    let start = Instant::now();
    let ret  = task_generator();
    block_on(ret);
    let elapsed = start.elapsed();
    print!("Task (Running 100 random numbers) Time elapsed {:?} \n", elapsed);

    let thread_start = Instant::now();
    thread_generator();
    let thread_elapsed = thread_start.elapsed();
    print!("Thread (Running 100 random numbers) Time elapsed {:?}\n", thread_elapsed);
}

fn rand_async() {

    let mut rng = rand::thread_rng();
    rng.gen::<u32>();
    
}

async fn task_generator() {

    let mut futures = Vec::new();
    
    for _i in 0..100{
        futures.push(tokio::task::spawn_blocking(|| {
            // This is running on a blocking thread.
            // Blocking here is ok.
            rand_async();
        }));
    }

    join_all(futures).await;

}



fn thread_generator() {
    
    let pool = ThreadPool::with_name("worker".into(), 1000);
    for _ in 0..100 {
        pool.execute(|| {
            assert_eq!(
                thread::current().name(),
                Some("worker")
            );
            let mut rng = rand::thread_rng();
            rng.gen::<u32>();
        });
    }
    pool.join();
}