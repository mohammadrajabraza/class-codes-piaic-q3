// multi-threading examples/scenario
use std::thread;
use std::time::Duration;

fn main() {    
    
    block_on(CollegeResult()); //This is to run Async examples/scenario starting from line 29
    // as we can't have two main() functions in one program

    //The following is multi-threading examples/scenario from line 10 to line 26
    let handle1 = thread::spawn(|| { //custom thread
        for i in 1..10 {
            println!("First custom Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle2 = thread::spawn(|| { //another custom thread
        for i in 1..10 {
            println!("Second custom Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle1.join().unwrap();
}


// Async examples/scenario
use std::io;
use futures::executor::block_on;

async fn get_sum(input: f32) -> f32 {
  input
}

async fn calculate_grade(a1:f32)   {
    if a1 > 55.0 {
        println!("Candidate is passed");
    }
    else{
        println!("Candidate is failed");
    }
    
}

async fn print_sum(sum: f32) {
    println!("Sum is {}", sum);
}

async fn find_sum_and_calc_grade(num: f32) {
    let sum = get_sum(num).await;
    calculate_grade(sum).await;
}

async fn CollegeResult() {
    let  mut input = String::new();
    println!("Enter sum of marks: ");
    io::stdin().read_line(&mut input).expect("please give me correct string number!");
    let num: f32 = input.trim().parse().expect("Please type a number");

    let f1 = find_sum_and_calc_grade(num);
    let f2 = print_sum(num);
    futures::join!(f2,f1);
}   
// fn main() {
//     block_on(CollegeResult());
// }