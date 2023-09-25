use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum cote {
    Gauche,
    Droit,
}

pub struct MovingRect {
    pub edge: cote,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}
impl MovingRect {
    pub fn new(edge: cote, x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            edge,
            x,
            y,
            width,
            height,
            speed,
        }
    }

    pub fn bouge(&mut self) {
        if (self.edge == cote::Gauche){
            self.x += self.speed;
        }
        else if (self.edge == cote::Droit){
            self.x -= self.speed;
        }

    }

    pub fn check_collision(&mut self, personnage_param : MovingRect) -> bool {
        if (self.x - personnage_param.x) < 20 {
            true
        }
        else {
            false
        }
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}