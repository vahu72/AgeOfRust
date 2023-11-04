
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Base {
    health : i32,
    damage : i32,
    revenue : i32,
    position : i32,
}
impl Base {
    pub fn new(health : i32, damage : i32, revenue : i32, position : i32) -> Base {
        Base {
            health,
            damage,
            revenue,
            position,
        }
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
}

// this trait represents every game component that interact with each other
pub trait Entity {
    fn get_health(&self) -> i32;
    fn get_damage(&self) -> i32;
    fn get_revenue(&self) -> i32;
    fn get_position(&self) -> i32;
    fn set_health(&mut self, new_health : i32);
}



impl Entity for Base {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_damage(&self) -> i32 {
        self.damage
    }


    fn get_revenue(&self) -> i32 {
        self.revenue
    }

    fn get_position(&self) -> i32 {
        self.position
    }

    fn set_health(&mut self, new_health: i32) {
        self.health = new_health;
    }
}


impl Entity for Fighter {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_damage(&self) -> i32 {
        self.damage
    }

    fn get_revenue(&self) -> i32 {
        self.revenue
    }

    fn get_position(&self) -> i32 {
        self.position
    }

    fn set_health(&mut self, new_health : i32) {
        self.health = new_health;
    }

}