use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};


//module 1
pub fn object1_start() {
    thread::spawn(|| {
        object1_run();
    });
}

fn object1_run() {
        // Your object1's thread logic here
        println!("Object1 is running!");
        thread::sleep(Duration::from_secs(1));
}

//module 2
pub fn object2_start() {
    thread::spawn(|| {
        object2_run();
    });
}

fn object2_run() {
        // Your object2's thread logic here
        println!("Object2 is running!");
        thread::sleep(Duration::from_secs(2));
}

// Define a separate function to increment the counter
fn increment_counter(counter: &Arc<Mutex<i32>>) {
    let mut data = counter.lock().unwrap(); // Lock the Mutex
    *data += 1; // Modify the shared data
    // Mutex is automatically released (unlocked) when `data` goes out of scope
}