use std::sync::mpsc;
use std::thread;

pub fn start() -> (thread::JoinHandle<()>, mpsc::Sender<String>) {
    let (sender, receiver) = mpsc::channel();
    let thread_handler = thread::spawn(|| {
        run(receiver);
    });
    (thread_handler, sender)
}
pub fn stop(handler: thread::JoinHandle<()>, sender: &mpsc::Sender<String>) {
    sender.send("stop".to_string()).unwrap();
    handler.join().unwrap();
}

pub fn ask_action(sender: &mpsc::Sender<String>) {
    sender.send("action asked".to_string()).unwrap();
}
fn run(receiver: mpsc::Receiver<String>) {
    //Doing something
    println!("Module actif is now actif");
    let mut is_running = true;
    let mut message;
    while is_running {
        message = receiver.recv().unwrap();

        if message == "action asked" {
            println!("Module actif is doing asked action");
        }
        else if message == "stop" {
            println!("Module actif is asked to stop");
            is_running = false;
        }
    }

}