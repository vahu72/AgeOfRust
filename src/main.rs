mod game_loop;
mod config;
pub mod gui;
use macroquad::prelude::*;
use crate::game_loop::{GameLoop};
use crate::game_loop::player::Player;


pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const GAME_NAME: &str = "Age of Rust";

#[macroquad::main(window_conf)]
async fn main() {
    let (mut gameloop, receiver) = GameLoop::new();
    gameloop.start(receiver);

    let (sender_keyboard, receiver_keyboard) = std::sync::mpsc::channel::<KeyCode>();
    let observer = game_loop::keyboard::KeyboardObserver::new(sender_keyboard);
    observer.start_observer();

    //creation of gui
    let graphics_manager = gui::GraphicsManager::new().await;
    let graphics_manager = match graphics_manager {
        Some(game_manager) => game_manager,
        // TODO : Gestion erreur
        None => todo!(),
    };

    let mut is_running = false;
    let mut player_left;
    let mut player_right;

    loop {
        if let Ok(key_code) = receiver_keyboard.try_recv() {
            if key_code == KeyCode::Escape && is_running {
                // Escape pressed
                is_running = false;
                gameloop.stop_game();
            } else if key_code == KeyCode::Escape && !is_running {
                // Escape pressed
                gameloop.stop();
                observer.stop_observer();
                break;
            }  else if key_code == KeyCode::Space && !is_running {
                // Space pressed
                is_running = true;
                gameloop.start_game();
            } else if key_code == KeyCode::Left && is_running {
                // Left key pressed
                gameloop.create_entity_left();
            } else if key_code == KeyCode::Right && is_running {
                // Right key pressed
                gameloop.create_entity_right();
            }
        }

        if is_running {
            player_left = gameloop.get_player_left();
            player_right = gameloop.get_player_right();
            let player_left = match player_left {
                Some(player) => player,
                None => {
                    println!("Error getting player left");
                    break;
                }
            };
            let player_right = match player_right {
                Some(player) => player,
                None => {
                    println!("Error getting player right");
                    break;
                }
            };

            // verification of the end of the game
            if player_left.get_health() <= 0 || player_right.get_health() <= 0 {
                is_running = false;
                gameloop.stop_game();
            }
            else {
                update_game(graphics_manager, player_left, player_right);
            }
        } else {
            graphics_manager.draw_title();
        }
    next_frame().await;
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: GAME_NAME.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

fn update_game(mut graphics_manager: gui::GraphicsManager, player_left: Player, player_right: Player) {

    graphics_manager.update(player_left, player_right);
}