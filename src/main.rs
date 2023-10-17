mod game_loop;
use macroquad::prelude::*;
use crate::game_loop::GameLoop;


pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const GAME_NAME: &str = "Rusty Corks";

#[macroquad::main(window_conf)]
async fn main() {
    let (mut gameloop, receiver) = GameLoop::new().await;

    gameloop.start(receiver);

    loop {
        //waiting ctrl+C
    }

    /*

    let mut running_game = false;
    let mut player_left = game_loop::player::Player::new(game_loop::player::Side::Left);
    let mut player_right = game_loop::player::Player::new(game_loop::player::Side::Right);
    let mut last_left_spawn_time : f64 = 0.0;
    let mut last_right_spawn_time : f64 = 0.0;

    // Créez un canal pour envoyer des événements de clavier et souris
    let (sender, receiver) = std::sync::mpsc::channel::<KeyCode>();
    let observer = game_loop::keyboard::KeyboardObserver::new(sender);
    observer.start_observer();

    // Gérer l'affichage graphique

    let graphics_manager = game_loop::gui::GraphicsManager::new().await;
    let graphics_manager = match graphics_manager {
        Some(game_manager) => game_manager,
        None => return,
    };

    // Affichage de la taille de la fenêtre
    println!("Window size: {}x{}", screen_width(), screen_height());

    println!("Starting loop");
    loop {
        clear_background(WHITE);
        let current_time = get_time();

        if let Ok(key_code) = receiver.try_recv() {
            if key_code == KeyCode::Escape {
                println!("Escape pressed");
                running_game = false;
            }
            else if key_code == KeyCode::Space {
                println!("Space pressed");
                running_game = true;
            }
            else if key_code == KeyCode::Left && running_game {
                let elapsed_time = current_time - last_left_spawn_time;
                if elapsed_time >= 1.0 {
                    player_left.create_entity(150, 100, game_loop::player::entity::Direction::Right, 100, 150, 1, 100);
                    last_left_spawn_time = current_time;
                }

            }
            else if key_code == KeyCode::Right && running_game {
                let elapsed_time = current_time - last_right_spawn_time;
                if elapsed_time >= 1.0 {
                    player_right.create_entity(100, 100, game_loop::player::entity::Direction::Left, 100, 150, 1, 685);
                    last_right_spawn_time = current_time;
                }

            }
        }

        if running_game {
            graphics_manager.draw_background_game();
            graphics_manager.draw_money(player_right.money, player_left.money);
            graphics_manager.draw_health(player_right.health, player_left.health);

            // Dessinez les entités des deux joueurs
            for left_entity in player_left.entities.iter_mut() {
                left_entity.set_position(left_entity.get_position() + left_entity.get_speed());

                // Collision entre une entité du joueur de gauche et la base adverse
                if left_entity.get_position() == 685 {
                    println!("Collision avec la base de droite");
                    // MAJ de la vie de l'entité concernée
                    left_entity.set_health(0);
                    // MAJ de la monnaie du joueur
                    player_left.money += left_entity.get_revenue();
                    // MAJ de la vie du joueur adverse
                    if player_right.health >= left_entity.get_damage() {
                        player_right.health -= left_entity.get_damage();
                    } else {
                        player_right.health = 0;
                    }
                }

                graphics_manager.draw_entity(true, left_entity.get_position() as f32);
            }

            for right_entity in player_right.entities.iter_mut() {
                right_entity.set_position(right_entity.get_position() - right_entity.get_speed());

                // Collision entre une entité du joueur de droite et la base adverse
                if right_entity.get_position() == 100 {
                    println!("Collision avec la base de gauche");
                    // MAJ de la vie de l'entité concernée
                    right_entity.set_health(0);
                    // MAJ de la monnaie du joueur
                    player_right.money += right_entity.get_revenue();
                    // MAJ de la vie du joueur adverse
                    if player_left.health >= right_entity.get_damage() {
                        player_left.health -= right_entity.get_damage();
                    } else {
                        player_left.health = 0;
                    }
                }

                graphics_manager.draw_entity(false, right_entity.get_position() as f32);
            }

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
                running_game = false;
            }
        } else {
            graphics_manager.draw_title();
        }
        next_frame().await;
    }
    */
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