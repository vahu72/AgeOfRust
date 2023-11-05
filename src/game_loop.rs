extern crate timer;
extern crate chrono;

use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use macroquad::time::get_time;
use macroquad::prelude::*;
use crate::game_loop::player::entity::Entity;
use crate::game_loop::player::Side;

pub mod keyboard;
pub mod player;

const REFRESH_PERIOD: i64 = 20; //ms

pub enum MessageType {
    Update,
    Stop,
    StopGame,
    StartGame,
    CreateEntityRight,
    CreateEntityLeft,
}

pub struct GameLoop {
    timer: timer::Timer,
    handler: Option<thread::JoinHandle<()>>,
    guard: Option<timer::Guard>,
    player_left: Arc<Mutex<player::Player>>,
    player_right: Arc<Mutex<player::Player>>,
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
            player_left: Arc::new(Mutex::new(player::Player::new(Side::Left))),
            player_right: Arc::new(Mutex::new(player::Player::new(Side::Right))),
            sender,
        },
         receiver)
    }

    pub fn start(&mut self, receiver: mpsc::Receiver<MessageType>) {
        let sender = self.sender.clone();

        let guard = self.timer.schedule_repeating(chrono::Duration::milliseconds(REFRESH_PERIOD), move || {
            // Timer ticked, notify game loop
            let send_result = sender.send(MessageType::Update);
            if let Err(e) = send_result {
                println!("Game loop: Error sending Update message: {}", e);
            }
        });
        self.guard = Some(guard);

        let player_left_clone = self.player_left.clone();
        let player_right_clone = self.player_right.clone();


        //start thread
        self.handler = Some(thread::spawn(|| {
            GameLoop::run(player_left_clone, player_right_clone, receiver);
        }));
    }

    pub fn get_player_left(&self) -> Option<player::Player> {
        let player_left_lock_ret = self.player_left.lock();
        match player_left_lock_ret {
            Ok(player_left) =>
                Some(player_left.clone()),
            Err(e) => {
                println!("Error getting player_left: {}", e);
                None
            }
        }
    }

    pub fn get_player_right(&self) -> Option<player::Player> {
        let player_right_lock_ret = self.player_right.lock();
        match player_right_lock_ret {
            Ok(player_right) =>
                Some(player_right.clone()),
            Err(e) => {
                println!("Error getting player_right: {}", e);
                None
            }
        }
    }

    pub fn stop(&mut self) {
        let send_result = self.sender.send(MessageType::Stop);
        if let Err(e) = send_result {
            println!("Game loop: Error sending stop message: {}", e);
        }
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

    pub fn create_entity_left(&mut self) {
        let send_result = self.sender.send(MessageType::CreateEntityLeft);
        if let Err(e) = send_result {
            println!("Game loop: Error sending stop message: {}", e);
        }
    }

    pub fn create_entity_right(&mut self) {
        let send_result = self.sender.send(MessageType::CreateEntityRight);
        if let Err(e) = send_result {
            println!("Game loop: Error sending stop message: {}", e);
        }
    }

    pub fn start_game(&mut self) {
        let send_result = self.sender.send(MessageType::StartGame);
        if let Err(e) = send_result {
            println!("Game loop: Error sending stop message: {}", e);
        }
    }

    pub fn stop_game(&mut self) {
        let send_result = self.sender.send(MessageType::StopGame);
        if let Err(e) = send_result {
            println!("Game loop: Error sending stop message: {}", e);
        }
    }

    fn run(player_left: Arc<Mutex<player::Player>>, player_right: Arc<Mutex<player::Player>>, receiver: mpsc::Receiver<MessageType>) {
        let mut is_running = true;
        let mut last_left_spawn_time: f64 = 0.0;
        let mut last_right_spawn_time: f64 = 0.0;
        while is_running {
            match receiver.try_recv() {
                Ok(message) => {
                    match message {
                        MessageType::Update => {
                            // locking mutexes
                            let player_right_lock_ret = player_right.lock();
                            let mut player_right = match player_right_lock_ret {
                                Ok(player_right) =>
                                   player_right,
                                Err(e) => {
                                    println!("Error getting player_right: {}", e);
                                    break;
                                }
                            };
                            let player_left_lock_ret = player_left.lock();
                            let mut player_left = match player_left_lock_ret {
                                Ok(player_left) =>
                                    player_left,
                                Err(e) => {
                                    println!("Error getting player_left: {}", e);
                                    break;
                                }
                            };

                            update_game(&mut player_left, &mut player_right);
                        }
                        MessageType::Stop => {
                            is_running = false;
                        }
                        MessageType::StartGame => {
                            // locking mutexes
                            let player_right_lock_ret = player_right.lock();
                            let mut player_right = match player_right_lock_ret {
                                Ok(player_right) =>
                                    player_right,
                                Err(e) => {
                                    println!("Error getting player_right: {}", e);
                                    break;
                                }
                            };
                            let player_left_lock_ret = player_left.lock();
                            let mut player_left = match player_left_lock_ret {
                                Ok(player_left) =>
                                    player_left,
                                Err(e) => {
                                    println!("Error getting player_left: {}", e);
                                    break;
                                }
                            };

                            player_left.reset();
                            player_right.reset();
                        }
                        MessageType::StopGame => {
                        }
                        MessageType::CreateEntityLeft => {
                            let current_time = get_time();
                            let elapsed_time = current_time - last_left_spawn_time;
                            if elapsed_time >= 0.5 {
                                let lock_ret = player_left.lock();
                                match lock_ret {
                                    Ok(mut player_left) =>
                                        player_left.create_entity(150, 100, 100, 150),
                                    Err(e) => {
                                        println!("Error getting player_left: {}", e);
                                    }
                                };
                                last_left_spawn_time = current_time;
                            }
                        }
                        MessageType::CreateEntityRight => {
                            let current_time = get_time();
                            let elapsed_time = current_time - last_right_spawn_time;
                            if elapsed_time >= 0.5 {
                                let lock_ret = player_right.lock();
                                match lock_ret {
                                    Ok(mut player_right) =>
                                        player_right.create_entity(150, 100, 100, 150),
                                    Err(e) => {
                                        println!("Error getting player_right: {}", e);
                                    }
                                };
                                last_right_spawn_time = current_time;
                            }
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

fn update_game(player1: &mut player::Player, player2: &mut player::Player) {
    // Incrementation de la position des entités des deux joueurs
    player1.fighters.iter_mut().for_each(|entity_player1| {
        entity_player1.set_position(entity_player1.get_position() + entity_player1.get_speed());
    });
    player2.fighters.iter_mut().for_each(|entity_player2| {
        entity_player2.set_position(entity_player2.get_position() - entity_player2.get_speed());
    });

    // Verification des collisions avec la base
    check_collision_adversary_base(player1, player2);
    check_collision_adversary_base(player2, player1);

    //verification des collisions entre entités
    check_collision_entities(player1, player2);

    player1.fighters.retain_mut(|entity_player1| {
        let mut to_retain = true;
        if entity_player1.get_health() <= 0 {
            to_retain = false;
        }
        to_retain
    });
    player2.fighters.retain_mut(|entity_player2| {
        let mut to_retain = true;
        if entity_player2.get_health() <= 0 {
            to_retain = false;
        }
        to_retain
    });
}

fn check_collision_adversary_base(defenser: &mut player::Player, attacker: &mut player::Player) {
    let (is_attacked, revenue) = attacker.check_colision_with_adversary_base();
    if is_attacked
    {
        // MAJ de la monnaie du joueur
        attacker.money += revenue;
        // MAJ de la vie du joueur adverse
        defenser.decrease_life(100);
    }
}

fn check_collision_entities(player_left: &mut player::Player, player_right: &mut player::Player) {
    for left_entity in player_left.fighters.iter_mut() {
        for right_entity in player_right.fighters.iter_mut() {

            // Collision entre deux entités
            if right_entity.get_position() - left_entity.get_position() <= 0 {
                println!("Collision");
                // MAJ de la vie des deux entités
                left_entity.set_health(left_entity.get_health() - right_entity.get_damage());
                right_entity.set_health(right_entity.get_health() - left_entity.get_damage());
                // MAJ de la monnaie des deux joueurs
                if left_entity.get_health() <= 0 {
                    player_right.money += left_entity.get_revenue();
                }
                if right_entity.get_health() <= 0 {
                    player_left.money += right_entity.get_revenue();
                }
            }
        }
    }
}