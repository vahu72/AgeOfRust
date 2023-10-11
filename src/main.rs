mod game_loop;
use macroquad::prelude::*;

#[macroquad::main("Background Image Example", window_conf)]
async fn main() {
    let mut running_game = false;
    let mut player_left = game_loop::player::Player::new(game_loop::player::Side::Left);
    let mut player_right = game_loop::player::Player::new(game_loop::player::Side::Right);

    // Créez un canal pour envoyer des événements de clavier et souris
    let (sender, receiver) = std::sync::mpsc::channel::<macroquad::input::KeyCode>();
    let observer = game_loop::keyboard::KeyboardObserver::new(sender);
    observer.start_observer();

    // Gérer l'affichage graphique

    let graphics_manager = game_loop::gui::GraphicsManager::new().await;
    let mut graphics_manager = match graphics_manager {
        Some(game_manager) => game_manager,
        None => return,
    };

    println!("Starting loop");
    loop {
        clear_background(WHITE);

        if let Ok(key_code) = receiver.try_recv() {
            if key_code == KeyCode::Escape {
                println!("Escape pressed");
                running_game = false;
            }
            else if key_code == KeyCode::Space {
                println!("Space pressed");
                running_game = true;
            }
            else if key_code == KeyCode::Left && running_game == true {
                player_left.create_entity(150, 100, game_loop::player::entity::Direction::Right, 100, 150, 1, 100);
            }
            else if key_code == KeyCode::Right && running_game == true {
                player_right.create_entity(100, 100, game_loop::player::entity::Direction::Left, 100, 150, 1, 685);
            }
        }

        if running_game == false {
            // Dessinez la texture du titre
            graphics_manager.draw_title();
        } else {
            // Dessinez la texture d'arrière-plan
            graphics_manager.draw_background_game();

            // Affichez la monnaie des deux joueurs
            graphics_manager.draw_money(player_right.money, player_left.money);

            // Affichez les points de vie des deux joueurs
            graphics_manager.draw_health(player_right.health, player_left.health);

            // Dessinez les entités des deux joueurs
            for left_entity in player_left.entities.iter_mut() {
                left_entity.set_position(left_entity.get_position() + left_entity.get_speed());
                graphics_manager.draw_entity(true, left_entity.get_position() as f32);
            }

            for right_entity in player_right.entities.iter_mut() {
                right_entity.set_position(right_entity.get_position() - right_entity.get_speed());
                graphics_manager.draw_entity(false, right_entity.get_position() as f32);
            }

            for left_entity in player_left.entities.iter_mut() {
                for right_entity in player_right.entities.iter_mut() {
                    if (left_entity.get_position() - right_entity.get_position()).abs() <= 1 {
                        println!("Collision");
                        // MAJ de la vie des deux entités
                        left_entity.set_health(left_entity.get_health() - right_entity.get_damage());
                        right_entity.set_health(right_entity.get_health() - left_entity.get_damage());
                        // MAJ de la monnaie des deux joueurs
                        if (left_entity.get_health() <= 0) {
                            player_right.money += left_entity.get_revenue();
                        }
                        if (right_entity.get_health() <= 0) {
                            player_left.money += right_entity.get_revenue();
                        }
                    }
                }
            }

            player_left.entities.retain_mut(|entity_left| {
                let mut to_retain = true;
                if (entity_left.get_health() <= 0) {
                    to_retain = false;
                }
                to_retain
            });

            player_right.entities.retain_mut(|entity_right| {
                let mut to_retain = true;
                if (entity_right.get_health() <= 0) {
                    to_retain = false;
                }
                to_retain
            });
        }
        next_frame().await;
    }
}
