use std::ops::Deref;
use std::clone::Clone;
mod entity;

#[derive(Clone)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone)]
pub struct Player {
    pub money: i32,
    pub side: Side,
    pub health: i32,
    pub entities: Vec<entity::Entity>,
}

impl Player {
    pub fn new(side : Side) -> Player {
        Player {
            money: 1000,
            side : side,
            health: 1000,
            entities: Vec::new(),
        }
    }

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_side(&self) -> &Side {
        &self.side
    }

    pub fn get_entities(&self) -> &Vec<entity::Entity> {
        &self.entities
    }

    pub fn get_entity(&self, index : usize) -> &entity::Entity {
        &self.entities[index]
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn get_entity_index_by_position(&self, position : i32) -> Option<usize> {
        for (index, entity) in self.entities.iter().enumerate() {
            if entity.get_position() == position {
                return Some(index);
            }
        }
        None
    }

    pub fn create_entity(&mut self, health : i32, damage : i32, direction : entity::Direction, cost : i32, revenue : i32, speed : i32, position : i32) {
        self.entities.push(entity::Entity::new(health, damage, direction, cost, revenue, speed, position));

    }

    pub fn remove_entity(&mut self, index : usize) {
        self.entities.remove(index);
    }

    pub fn decrease_life(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
            //TODO: notify death
        }
    }

    pub fn increase_money(&mut self, amount: i32) {
        self.money += amount;
    }

    pub fn decrease_money(&mut self, amount: i32) -> bool {
        if self.money > 0 {
            self.money -= amount;
            true
        }
        else {
            false
        }
    }


}
