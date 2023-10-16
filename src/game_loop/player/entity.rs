use std::clone::Clone;


#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
pub struct Entity {
    health : i32,
    damage : i32,
    direction : Direction,
    cost : i32,
    revenue : i32,
    speed : i32,
    position : i32,
}

impl Entity {
    pub fn new(health : i32, damage : i32, direction : Direction, cost : i32, revenue : i32, speed : i32, position : i32) -> Entity {
        Entity {
            health : health,
            damage : damage,
            direction : direction,
            cost : cost,
            revenue : revenue,
            speed : speed,
            position : position,
        }
    }
    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_cost(&self) -> i32 {
        self.cost
    }

    pub fn get_revenue(&self) -> i32 {
        self.revenue
    }

    pub fn get_speed(&self) -> i32 {
        self.speed
    }

    pub fn get_position(&self) -> i32 {
        self.position
    }

    pub fn set_health(&mut self, health : i32) {
        self.health = health;
    }

    pub fn set_position(&mut self, position : i32) {
        self.position = position;
    }
}

