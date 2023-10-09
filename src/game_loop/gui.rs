use macroquad::prelude::{load_texture, Texture2D};
use macroquad::prelude::{draw_rectangle, RED, GREEN};
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

    pub fn draw_entity(&self, side : bool) {
        if (side == true) {
            // Dessinez un rectangle rouge à gauche de l'écran dans le cas du joueur de gauche
            draw_rectangle(0.0, 0.0, 100.0, 100.0, RED);
        }
        else {
            // Dessinez un rectangle vert à droite de l'écran dans le cas du joueur de droite
            draw_rectangle(0.0, 0.0, 100.0, 100.0, GREEN);
        }
    }
}