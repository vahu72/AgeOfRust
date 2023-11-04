pub mod entity;

use crate::config::{BASE_LEFT_POSITION, BASE_RIGHT_POSITION, ENTITY_SPEED};
use std::clone::Clone;
use crate::game_loop::player::entity::Entity;

#[derive(PartialEq)]
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
    pub fighters: Vec<entity::Fighter>,
}

impl Player {
    pub fn new(side : Side) -> Player {
        Player {
            money: 250,
            side,
            health: 1000,
            fighters: Vec::new(),
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

    pub fn get_entities(&mut self) -> &mut Vec<entity::Fighter> {
        &mut self.fighters
    }

    pub fn get_entity(&self, index : usize) -> &entity::Fighter {
        &self.fighters[index]
    }

    pub fn get_entity_count(&self) -> usize {
        self.fighters.len()
    }

    pub fn get_entity_index_by_position(&self, position : i32) -> Option<usize> {
        for (index, fighter) in self.fighters.iter().enumerate() {
            if fighter.get_position() == position {
                return Some(index);
            }
        }
        None
    }

    // Too many arguments : autorisé car vu avec M. JOUAULT
    #[allow(clippy::too_many_arguments)]
    pub fn create_entity(&mut self, health : i32, damage : i32, cost : i32, revenue : i32) {
        let position : i32;
        if self.money >= cost {
            if self.side == Side::Right {
                position = BASE_RIGHT_POSITION;
            }
            else {
                position = BASE_LEFT_POSITION;
            }
            self.fighters.push(entity::Fighter::new(health, damage, cost, revenue, ENTITY_SPEED, position));
            self.money -= cost;
        }
    }

    pub fn remove_entity(&mut self, entity_to_remove : &entity::Fighter) {
        self.fighters.retain(|entity| entity != entity_to_remove);
    }

    pub fn decrease_life(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
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

    pub fn check_colision_with_adversary_base(&mut self) -> (bool, i32) {
        for fighter in self.fighters.iter_mut() {
            if BASE_LEFT_POSITION - fighter.get_position() > 0 && self.side == Side::Right ||
                BASE_RIGHT_POSITION - fighter.get_position() < 0 && self.side == Side::Left
            {
                // MAJ de la vie de l'entité concernée
                fighter.set_health(0);
                return (true, fighter.get_revenue());
            }
        }
        (false, 0)
    }
}
