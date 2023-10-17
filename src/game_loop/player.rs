pub mod entity;
use std::clone::Clone;
use crate::game_loop::player::entity::{ENTITY_LEFT_POSITION, ENTITY_RIGHT_POSITION};

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
            money: 250,
            side,
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

    pub fn get_entities(&mut self) -> &mut Vec<entity::Entity> {
        &mut self.entities
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

    // Too many arguments : autorisé car vu avec M. JOUAULT
    #[allow(clippy::too_many_arguments)]
    pub fn create_entity(&mut self, health : i32, damage : i32, direction : entity::Direction, cost : i32, revenue : i32, speed : i32) {
        let mut position : i32 = ENTITY_RIGHT_POSITION;
        if self.money >= cost {
            if direction == entity::Direction::Right {
                position = ENTITY_LEFT_POSITION;
                }
            self.entities.push(entity::Entity::new(health, damage, direction, cost, revenue, speed, position));
            self.money -= cost;
        }
    }

    pub fn remove_entity(&mut self, entity_to_remove : &entity::Entity) {
        self.entities.retain(|entity| entity != entity_to_remove);
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

    pub fn check_colision_with_adversary_base(&mut self, base_position: i32) -> (bool, i32) {
        for entity in self.entities.iter_mut() {
            if base_position - entity.get_position() > 0 && *entity.get_direction() == entity::Direction::Left ||
                base_position - entity.get_position() < 0 && *entity.get_direction() == entity::Direction::Right
            {
                // MAJ de la vie de l'entité concernée
                entity.set_health(0);
                return (true, entity.get_revenue());
            }
        }
        (false, 0)
    }
}
