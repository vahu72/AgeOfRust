
mod tasks;

use std::thread;
use std::time::Duration;

fn main() {
    // Initialize and launch object1's thread
    tasks::object1_start();

    // Initialize and launch object2's thread
    tasks::object2_start();

    // Let the threads run for a while
    thread::sleep(Duration::from_secs(5));
}

//utilisation mutex :

//let counter = Arc::new(Mutex::new(0));
//let counter = Arc::clone(&counter);
//*data += 1; // Modify the shared data
//*data += 1; // Modify the shared data