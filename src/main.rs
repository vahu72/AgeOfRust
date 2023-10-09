mod game_loop;
use macroquad::prelude::*;

#[macroquad::main("Background Image Example", window_conf)]
async fn main() {
    let mut show_title = true; // Initial state: afficher la texture du titre

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

        if let Ok(key_code) = receiver.recv() {
            if key_code == KeyCode::Escape {
                show_title = true;
            }
        }

        if show_title {
            // Dessinez la texture du titre
            graphics_manager.draw_title();

        } else {
            // Dessinez la texture d'arrière-plan
            graphics_manager.draw_background_game();
        }

        next_frame().await;
    }
}
