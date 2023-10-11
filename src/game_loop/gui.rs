use macroquad::prelude::{load_texture, Texture2D};
use macroquad::prelude::{draw_rectangle, draw_text, RED, BLUE, BLACK};
pub struct GraphicsManager {
    pub main_menu: Texture2D,
    pub game_background: Texture2D,
}

impl GraphicsManager {
    pub async fn new() -> Option<GraphicsManager> {
        let main_menu = load_texture("flash_age_of_war_screenshot.png").await;
        let game_background = load_texture("ageofwar.png").await;

        match (main_menu, game_background) {
            (Ok(main_menu), Ok(game_background)) => Some(GraphicsManager {
                main_menu,
                game_background,
            }),
            (Err(_), _) => {
                println!("Error loading texture: flash_age_of_war_screenshot.png");
                None
            }
            (_, Err(_)) => {
                println!("Error loading texture: ageofwar.png");
                None
            }
        }
    }

    pub fn draw_title(&self) {
        // Dessinez la texture du titre
        macroquad::prelude::draw_texture_ex(
            self.main_menu,
            0.0,
            0.0,
            macroquad::prelude::WHITE,
            macroquad::prelude::DrawTextureParams {
                dest_size: Some(macroquad::prelude::vec2(macroquad::prelude::screen_width(), macroquad::prelude::screen_height())),
                ..Default::default()
            },
        );
    }

    pub fn draw_background_game(&self) {
        // Dessinez la texture d'arrière-plan
        macroquad::prelude::draw_texture_ex(
            self.game_background,
            0.0,
            0.0,
            macroquad::prelude::WHITE,
            macroquad::prelude::DrawTextureParams {
                dest_size: Some(macroquad::prelude::vec2(macroquad::prelude::screen_width(), macroquad::prelude::screen_height())),
                ..Default::default()
            },
        );
    }

    pub fn draw_entity(&self, side : bool, position : f32) {
        if (side == true) {
            // Dessinez un rectangle rouge à gauche de l'écran dans le cas du joueur de gauche
            draw_rectangle(position, 505.0, 12.0, 50.0, RED);
        }
        else {
            // Dessinez un rectangle vert à droite de l'écran dans le cas du joueur de droite
            draw_rectangle(position, 505.0, 12.0, 50.0, BLUE);
        }
    }

    pub fn draw_money(&self, player_right_money : i32, player_left_money : i32) {
        // Affichez la monnaie des deux joueurs
        let money_right_message = format!("Money: {}$", player_right_money);
        let money_left_message= format!("Money: {}$", player_left_money);
        draw_text(&money_left_message, 10.0, 30.0, 30.0, BLACK);
        draw_text(&money_right_message, 650.0, 30.0, 30.0, BLACK);
    }

    pub fn draw_health(&self, player_right_health : i32, player_left_health: i32) {
        // Affichez la monnaie des deux joueurs
        let health_right_message = format!("Health: {}", player_right_health);
        let health_left_message = format!("Health: {}", player_left_health);
        draw_text(&health_left_message, 8.0, 60.0, 30.0, RED);
        draw_text(&health_right_message, 640.0, 60.0, 30.0, RED);
    }
}