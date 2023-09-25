use macroquad::prelude::*;

#[macroquad::main("Background Image Example", window_conf)]
async fn main() {
    // Chargez l'image du titre à partir du fichier "flash_age_of_war_screenshot.png" dans le répertoire de travail.
    let title_texture = load_texture("flash_age_of_war_screenshot.png").await.expect("Failed to load title image");

    // Chargez l'image d'arrière-plan à partir du fichier "ageofwar.png" dans le répertoire de travail.
    let background_texture = load_texture("ageofwar.png").await.expect("Failed to load background image");

    let mut show_title = true; // Initial state: afficher la texture du titre

    loop {
        clear_background(WHITE);

        if is_mouse_button_down(MouseButton::Left) {
            // Obtenez la position actuelle de la souris
            let mouse_pos_tuple = mouse_position();
            let mouse_pos = vec2(mouse_pos_tuple.0, mouse_pos_tuple.1);
            // Définissez la zone de détection comme un rectangle
            let area_rect_play = Rect::new(350.0, 250.0, 100.0, 45.0);
            let area_rect_stop = Rect::new(0.0, 0.0, 20.0, 20.0);

            // Vérifiez si la position de la souris se trouve à l'intérieur de la zone de détection
            if area_rect_play.contains(mouse_pos) {
                show_title = false;
            }
            if area_rect_stop.contains(mouse_pos) {
                show_title = true;
            }
        }

        if show_title {
            // Dessinez la texture du titre
            draw_texture_ex(
                title_texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            //draw_rectangle(350.0, 250.0, 100.0, 45.0, RED);
        } else {
            // Dessinez la texture d'arrière-plan
            draw_texture_ex(
                background_texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }

        next_frame().await;
    }
}
