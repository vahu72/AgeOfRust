extern crate timer;
extern crate chrono;
use std::sync::mpsc;
use crate::game_loop::player::Side;

mod gui;
mod keyboard;
mod player;

const REFRESH_PERIOD: i64 = 1000; //ms

pub enum MessageType {
    CreateEntity,
    Update,
    Stop,
}

pub struct GameLoop {
    timer: timer::Timer,
    player_left: player::Player,
    player_right: player::Player,
    sender: mpsc::Sender<MessageType>,
    receiver:  mpsc::Receiver<MessageType>,
}

impl GameLoop {
    pub fn new() -> Self {
        //creation of the mq
        let (sender, receiver)= mpsc::channel::<MessageType>();

        GameLoop {
            timer: timer::Timer::new(),
            player_left: player::Player::new(Side::Left),
            player_right: player::Player::new(Side::Right),
            sender,
            receiver,
        }
    }

    pub fn start(&mut self) {
        let sender = self.sender.clone();
        println!("Timer fired!");
        let _guard = self.timer.schedule_repeating(chrono::Duration::milliseconds(REFRESH_PERIOD), move || {
            update_game(&sender);
        });
        //TODO: start modules

        self.run();


    }

    pub fn stop(&mut self) {
        /*self.game_timer.game_timer_stop();
        self.keyboard.keyboard_stop();
        self.gui.gui_stop();
        self.player.player_stop();*/
    }

    pub fn ask_action(&mut self) {
        //send MessageType CreateEntity on the mq
        self.sender.send(MessageType::CreateEntity).unwrap();


    }

    fn run (&mut self) {
        let mut is_running = true;
        let mut message;
        while is_running {
            message = self.receiver.recv().unwrap();
            match message {
                MessageType::CreateEntity => {
                    println!("ouais trop cool ca marche bien \n");
                },
                MessageType::Update => {
                    println!("ouais trop cool ca update bien \n");
                },
                MessageType::Stop => {
                    println!("... non trop nul ca fonctionne pas \n");
                    is_running = false;
                },
            }
        }
    }
}
pub fn update_game(sender: &mpsc::Sender<MessageType>) {
    //TODO : update the game by sending message on the mq
    sender.send(MessageType::Update).unwrap();
}



/*pub fn game_loop_start() -> (thread::JoinHandle<()>, mpsc::Sender<String>) {
    //Starting the keyboard module
    let (keyboard_handler, keyboard_sender) = keyboard::keyboard_start();

    //Starting the game loop thread
    let (sender, receiver) = mpsc::channel();
    let thread_handler = thread::spawn(|| {
        run(receiver);
    });
    (thread_handler, sender)
}
pub fn game_loop_stop(handler: thread::JoinHandle<()>, sender: &mpsc::Sender<String>) {
    sender.send("stop".to_string()).unwrap();
    handler.join().unwrap();
}



//pub fn ask_action(sender: &mpsc::Sender<String>) {
//    sender.send("action asked".to_string()).unwrap();
//}
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

}*/
