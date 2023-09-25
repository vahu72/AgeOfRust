use macroquad::prelude::*;

pub struct MovingRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}
impl MovingRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    pub fn move_right(&mut self) {
        self.x += self.speed;
    }

    pub fn get_coordinates(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}