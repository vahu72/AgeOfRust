mod game_loop;
use macroquad::prelude::*;

#[macroquad::main("Background Image Example", window_conf)]
async fn main() {
    let mut running_game = false;

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
                println!("Left pressed so print");
                graphics_manager.draw_entity(true);
            }
            else if key_code == KeyCode::Right && running_game == true {
                graphics_manager.draw_entity(false);
            }
        }

        if running_game == false {
            // Dessinez la texture du titre
            graphics_manager.draw_title();

        } else {
            // Dessinez la texture d'arrière-plan
            graphics_manager.draw_background_game();
            graphics_manager.draw_entity(false);
        }

        next_frame().await;
    }
}
