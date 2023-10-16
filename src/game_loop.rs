extern crate timer;
extern crate chrono;

use std::sync::mpsc;
use std::thread;
use crate::game_loop::player::Side;

mod gui;
mod keyboard;
mod player;

const REFRESH_PERIOD: i64 = 1000; //ms

pub enum MessageType {
    Update,
    Stop,
}

pub struct GameLoop {
    timer: timer::Timer,
    handler: Option<thread::JoinHandle<()>>,
    guard: Option<timer::Guard>,
    player_left: player::Player,
    player_right: player::Player,
    sender: mpsc::Sender<MessageType>,
}

impl GameLoop {
    pub fn new() -> (Self, mpsc::Receiver<MessageType>) {
        //creation of the mq
        let (sender, receiver) = mpsc::channel::<MessageType>();
        let receiver = receiver;
        (GameLoop {
            timer: timer::Timer::new(),
            handler: None,
            guard: None,
            player_left: player::Player::new(Side::Left),
            player_right: player::Player::new(Side::Right),
            sender,
        },
         receiver)
    }

    pub fn start(&mut self, receiver: mpsc::Receiver<MessageType>)
    {
        let sender = self.sender.clone();
        println!("Timer fired!");
        let guard = self.timer.schedule_repeating(chrono::Duration::milliseconds(REFRESH_PERIOD), move || {
            sender.send(MessageType::Update).unwrap();
        });
        self.guard = Some(guard);

        let player_left_clone = self.player_left.clone();
        let player_right_clone = self.player_right.clone();

        // sleep
        //start thread
        self.handler = Some(thread::spawn(|| {
            GameLoop::run(player_left_clone, player_right_clone, receiver);
        }));
    }

    pub fn stop(&mut self) {
        self.sender.send(MessageType::Stop).unwrap();
        if self.handler.is_some() {
            if let Some(thread) = self.handler.take() {
                let _ = thread.join();
            }
            // Cleanup the timer after stopping the thread
            if let Some(guard) = self.guard.take() {
                drop(guard);
            }
        }
    }

    fn run(_player_left: player::Player, _player_right: player::Player, receiver: mpsc::Receiver<MessageType>) {
        let mut is_running = true;
        while is_running {
            match receiver.try_recv() {
                Ok(message) => {
                    match message {
                        MessageType::Update => {
                            println!("ouais trop cool ca update bien \n");
                        }
                        MessageType::Stop => {
                            println!("... non trop nul ca fonctionne pas \n");
                            is_running = false;
                        }
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Receiver is disconnected, exiting.");
                    is_running = false;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    // No message received, continue running
                }
            }
        }
    }
}