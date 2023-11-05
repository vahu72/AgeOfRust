use crate::config::{ DEFAULT_HEALTH};

#[derive(PartialEq)]
#[derive(Clone)]
pub struct Base {
    pub(crate) health : i32,
    position : i32,
}
impl Base {
    pub fn new(position : i32) -> Base {
        Base {
            health : DEFAULT_HEALTH,
            position,
        }
    }
    pub fn reset(&mut self) {
        self.health = DEFAULT_HEALTH;
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
pub struct Fighter {
    health : i32,
    damage : i32,
    cost : i32,
    revenue : i32,
    speed : i32,
    position : i32,
}
impl Fighter {
    pub fn new(health : i32, damage : i32, cost : i32, revenue : i32, speed : i32, position : i32) -> Fighter {
        Fighter {
            health,
            damage,
            cost,
            revenue,
            speed,
            position,
        }
    }
    pub fn get_speed(&self) -> i32 {
        self.speed
    }
    pub fn get_cost(&self) -> i32 {
        self.cost
    }
    pub fn set_position(&mut self, new_position : i32) {
        self.position = new_position;
    }

    pub fn get_damage(&self) -> i32 {
        self.damage
    }

    pub fn get_revenue(&self) -> i32 {
        self.revenue
    }
}

// this trait represents every game component that interact with each other
pub trait Entity {
    fn get_health(&self) -> i32;
    fn get_position(&self) -> i32;
    fn set_health(&mut self, new_health : i32);
}



impl Entity for Base {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_position(&self) -> i32 {
        self.position
    }

    fn set_health(&mut self, new_health: i32) {
        self.health = new_health;
        if self.health < 0 {
            self.health = 0;
        }
    }
}


impl Entity for Fighter {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_position(&self) -> i32 {
        self.position
    }

    fn set_health(&mut self, new_health : i32) {
        self.health = new_health;
    }

}