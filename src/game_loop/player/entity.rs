use std::clone::Clone;

#[derive(PartialEq)]
#[derive(Clone)]
pub struct Entity {
    health : i32,
    damage : i32,
    cost : i32,
    revenue : i32,
    speed : i32,
    position : i32,
}

impl Entity {
    pub fn new(health : i32, damage : i32, cost : i32, revenue : i32, speed : i32, position : i32) -> Entity {
        Entity {
            health,
            damage,
            cost,
            revenue,
            speed,
            position,
        }
    }
    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
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

    pub fn set_health(&mut self, new_health : i32) {
        self.health = new_health;
    }

    pub fn set_position(&mut self, new_position : i32) {
        self.position = new_position;
    }
}

