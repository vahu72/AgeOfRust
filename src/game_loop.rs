extern crate timer;
extern crate chrono;

use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use macroquad::time::get_time;
use macroquad::prelude::*;
use crate::game_loop::player::entity::ENTITY_SPEED;
use crate::game_loop::player::Side;

pub mod keyboard;
pub mod player;

const REFRESH_PERIOD: i64 = 1000; //ms

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
    //gui : gui::GraphicsManager,
}

impl GameLoop {
    pub async fn new() -> (Self, mpsc::Receiver<MessageType>) {
        //creation of the mq
        let (sender, receiver) = mpsc::channel::<MessageType>();
        let receiver = receiver;

        println!("GameLoop new");
        (GameLoop {
            timer: timer::Timer::new(),
            handler: None,
            guard: None,
            player_left: Arc::new(Mutex::new(player::Player::new(Side::Left))),
            player_right: Arc::new(Mutex::new(player::Player::new(Side::Right))),
            sender,
            //gui: graphics_manager,
        },
         receiver)
    }

    pub fn start(&mut self, receiver: mpsc::Receiver<MessageType>)
    {
        let sender = self.sender.clone();
        println!("GameLoop start!");
        let guard = self.timer.schedule_repeating(chrono::Duration::milliseconds(REFRESH_PERIOD), move || {
            //println!("Timer ticked!");
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

    pub fn get_player_left(&self) -> player::Player {
        let player_left = self.player_left.lock().unwrap();
        player_left.clone()
    }

    pub fn get_player_right(&self) -> player::Player {
        let player_right = self.player_right.lock().unwrap();
        player_right.clone()
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

    pub fn create_entity_left(&mut self) {
        self.sender.send(MessageType::CreateEntityLeft).unwrap();
    }

    pub fn create_entity_right(&mut self) {
        self.sender.send(MessageType::CreateEntityRight).unwrap();
    }

    pub fn start_game(&mut self) {
        self.sender.send(MessageType::StartGame).unwrap();
    }

    pub fn stop_game(&mut self) {
        self.sender.send(MessageType::StopGame).unwrap();
    }

    fn run(player_left: Arc<Mutex<player::Player>>, player_right: Arc<Mutex<player::Player>>, receiver: mpsc::Receiver<MessageType>) {
        let mut is_running = true;
        let mut last_left_spawn_time : f64 = 0.0;
        let mut last_right_spawn_time : f64 = 0.0;
        while is_running {
            match receiver.try_recv() {
                Ok(message) => {
                    match message {
                        MessageType::Update => {
                            // locking mutexes
                            let mut player_left = player_left.lock().unwrap();
                            let mut player_right = player_right.lock().unwrap();
                            // Dessinez les entités des deux joueurs
                            /*for left_entity in player_left.entities.iter_mut() {
                                left_entity.set_position(left_entity.get_position() + left_entity.get_speed());

                                // Collision entre une entité du joueur de gauche et la base adverse
                                if left_entity.get_position() == 685 {
                                    println!("Collision avec la base de droite");
                                    // MAJ de la vie de l'entité concernée
                                    left_entity.set_health(0);
                                    // MAJ de la monnaie du joueur
                                    //player_left.money += left_entity.get_revenue(); //FIXME:
                                    // MAJ de la vie du joueur adverse
                                    if player_right.health >= left_entity.get_damage() {
                                        player_right.health -= left_entity.get_damage();
                                    } else {
                                        player_right.health = 0;
                                    }
                                }

                            }*/

                            /*for right_entity in player_right.entities.iter_mut() {
                                right_entity.set_position(right_entity.get_position() - right_entity.get_speed());

                                // Collision entre une entité du joueur de droite et la base adverse
                                if right_entity.get_position() == 100 {
                                    println!("Collision avec la base de gauche");
                                    // MAJ de la vie de l'entité concernée
                                    right_entity.set_health(0);
                                    // MAJ de la monnaie du joueur
                                    //player_right.money += right_entity.get_revenue(); //FIXME:
                                    // MAJ de la vie du joueur adverse
                                    if player_left.health >= right_entity.get_damage() {
                                        player_left.health -= right_entity.get_damage();
                                    } else {
                                        player_left.health = 0;
                                    }
                                }
                            }*/
                            let (is_attacked, revenue) = player_left.check_colision_with_adversary_base(685);
                            if is_attacked
                            {
                                println!("Collision avec la base de droite");
                                // MAJ de la vie de l'entité concernée
                                player_left.decrease_life(100);
                                // MAJ de la monnaie du joueur
                                player_left.money += revenue;
                                // MAJ de la vie du joueur adverse
                                player_right.decrease_life(100);
                            }

                            let (is_attacked, revenue) = player_right.check_colision_with_adversary_base(100);
                            if is_attacked
                            {
                                println!("Collision avec la base de gauche");
                                // MAJ de la vie de l'entité concernée
                                player_right.decrease_life(100);
                                // MAJ de la monnaie du joueur
                                player_right.money += revenue;
                                // MAJ de la vie du joueur adverse
                                player_left.decrease_life(100);
                            }

                            GameLoop::check_collision_entities(&mut player_left, &mut player_right);

                            player_left.entities.retain_mut(|entity_left| {
                                let mut to_retain = true;
                                if entity_left.get_health() <= 0 {
                                    to_retain = false;
                                }
                                to_retain
                            });

                            player_right.entities.retain_mut(|entity_right| {
                                let mut to_retain = true;
                                if entity_right.get_health() <= 0 {
                                    to_retain = false;
                                }
                                to_retain
                            });

                            if player_right.get_health() <= 0 || player_left.get_health() <= 0  {
                                println!("game ended !")
                            }
                        }
                        MessageType::Stop => {
                            println!("... non trop nul ca fonctionne pas \n");
                            is_running = false;
                        }
                        MessageType::StartGame => {
                            println!("Espace pressed");
                        }
                        MessageType::StopGame => {
                            println!("Escape pressed");
                        }
                        MessageType::CreateEntityLeft => {
                            println!("Left pressed");
                            let current_time = get_time();
                            let elapsed_time = current_time - last_left_spawn_time;
                            if elapsed_time >= 1.0 {
                                let mut player_left = player_left.lock().unwrap();
                                player_left.create_entity(150, 100, player::entity::Direction::Right, 100, 150, ENTITY_SPEED);
                                last_left_spawn_time = current_time;
                            }
                        }
                        MessageType::CreateEntityRight => {
                            println!("Right pressed");
                            let current_time = get_time();
                            let elapsed_time = current_time - last_right_spawn_time;
                            if elapsed_time >= 1.0 {
                                let mut player_right = player_right.lock().unwrap();
                                player_right.create_entity(100, 100, player::entity::Direction::Left, 100, 150, ENTITY_SPEED);
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

    fn check_collision_entities(player_left: &mut player::Player, player_right: &mut player::Player) {
        for left_entity in player_left.entities.iter_mut() {
            for right_entity in player_right.entities.iter_mut() {

                // Collision entre deux entités
                if (left_entity.get_position() - right_entity.get_position()).abs() <= 1 {
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
}