#![allow(unused_imports)]
use futures::prelude::*;
use tokio::prelude::*;
use futures::executor::block_on;
use std::thread;
use std::time::Duration;

async fn motion_sensor()
{
    thread::sleep(Duration::from_millis(100));//Supposing a person will enter room after 200ms 
    iot_home();    
    cleaner_bot().await;
}
async fn cleaner_bot()
{
    println!("The cleaner bot starts cleaning");
    thread::sleep(Duration::from_millis(1000));//After 100ms cleaner bot finished cleaning and will return to it's pod
    println!("The house is cleaned, the bot returned to his pod");
}

fn iot_home(){

    let turn_on_bulb = thread::spawn(move ||{
        thread::sleep(Duration::from_millis(100));
        println!("Bulb turned on\nA motion has been detected");
    }); 
    
    let turn_on_fan = thread::spawn(move ||{
        thread::sleep(Duration::from_millis(50));
        println!("Fan turned on\n");
    }); 
    
    let turn_off_bulb = thread::spawn(move ||{
        thread::sleep(Duration::from_millis(1100));
        println!("Room bulb is off/turned off");
    });

    let turn_off_fan = thread::spawn(move ||{
        thread::sleep(Duration::from_millis(100));
        println!("Fan turned off\n");
    }); 
}

#[tokio::main]
pub async fn main() {
    println!("Program starts");
    
    println!("Async and Multithreading Functions Example");
    
    println!("IoT House");
    
    
  
    let motion_sensor = motion_sensor();
    // cleaner_bot().await;
    block_on(motion_sensor);


    println!("Program ended");
}