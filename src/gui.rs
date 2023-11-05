use macroquad::prelude::{load_texture, Texture2D};
use macroquad::prelude::{draw_rectangle, draw_text, RED, BLUE, BLACK};
use std::clone::Clone;
use crate::game_loop::player::entity::Entity;

use crate::game_loop::player::Player;

const LEFT_PLAYER_MENU_X : f32 = 10.0;
const LEFT_PLAYER_MENU_Y : f32 = 30.0;
const RIGHT_PLAYER_MENU_X : f32 = 1100.0;
const RIGHT_PLAYER_MENU_Y : f32 = 30.0;
const ENTITY_WIDTH : f32 = 14.0;
const ENTITY_HEIGHT : f32 = 58.0;
const ENTITY_POSITION_Y : f32 = 610.0;

#[derive(Clone, Copy)]
pub struct GraphicsManager {
    pub main_menu: Texture2D,
    pub game_background: Texture2D,
}

impl GraphicsManager {
    pub async fn new() -> Option<GraphicsManager> {
        let main_menu = load_texture("main_menu.png").await;
        let game_background = load_texture("background.png").await;

        match (main_menu, game_background) {
            (Ok(main_menu), Ok(game_background)) => Some(GraphicsManager {
                main_menu,
                game_background,
            }),
            (Err(_), _) => {
                println!("Error loading texture: main_menu.png");
                None
            }
            (_, Err(_)) => {
                println!("Error loading texture: background.png");
                None
            }
        }
    }

    pub fn draw_title(&self) {
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

    fn draw_entity(&self, side : bool, position : f32) {
        if side {
            draw_rectangle(position, ENTITY_POSITION_Y, ENTITY_WIDTH, ENTITY_HEIGHT, RED);
        }
        else {
            draw_rectangle(position, ENTITY_POSITION_Y, ENTITY_WIDTH, ENTITY_HEIGHT, BLUE);
        }
    }

    fn draw_money(&self, player_right_money : i32, player_left_money : i32) {
        let money_right_message = format!("Money: ${}", player_right_money);
        let money_left_message= format!("Money: ${}", player_left_money);
        draw_text(&money_left_message, LEFT_PLAYER_MENU_X, LEFT_PLAYER_MENU_Y, 30.0, BLACK);
        draw_text(&money_right_message, RIGHT_PLAYER_MENU_X, RIGHT_PLAYER_MENU_Y, 30.0, BLACK);
    }

    fn draw_health(&self, player_right_health : i32, player_left_health: i32) {
        let health_right_message = format!("Health: {}", player_right_health);
        let health_left_message = format!("Health: {}", player_left_health);
        draw_text(&health_left_message, LEFT_PLAYER_MENU_X - 2.0, 2.0 * RIGHT_PLAYER_MENU_Y, 30.0, RED);
        draw_text(&health_right_message, RIGHT_PLAYER_MENU_X - 2.0, 2.0 * RIGHT_PLAYER_MENU_Y, 30.0, RED);
    }

    pub fn update(&mut self, player_left : Player, player_right : Player) {
        self.draw_background_game();
        self.draw_money(player_right.money, player_left.money);
        self.draw_health(player_right.base.health, player_left.base.health);
        for entity in player_left.fighters.iter() {
            self.draw_entity(true, entity.get_position() as f32);
        }
        for entity in player_right.fighters.iter() {
            self.draw_entity(false, entity.get_position() as f32);
        }
    }
}