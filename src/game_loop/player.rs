pub mod entity;

use crate::config::{ENTITY_SPEED, DEFAULT_MONEY, BASE_LEFT_POSITION, BASE_RIGHT_POSITION};
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
    pub base: entity::Base,
    pub fighters: Vec<entity::Fighter>,
}

impl Player {
    pub fn new(side : Side) -> Player {
        let position = match side {
            Side::Left => BASE_LEFT_POSITION,
            Side::Right => BASE_RIGHT_POSITION,
        };
        Player {
            money: DEFAULT_MONEY,
            side,
            base: entity::Base::new(position),
            fighters: Vec::new(),
        }
    }

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn get_health(&self) -> i32 {
        self.base.get_health()
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

    pub fn reset(&mut self) {
        self.base.reset();
        self.money = DEFAULT_MONEY;
        self.fighters.clear();
    }

    pub fn create_entity(&mut self, health : i32, damage : i32, cost : i32, revenue : i32) {
        if self.money >= cost {
            self.fighters.push(entity::Fighter::new(health, damage, cost, revenue, ENTITY_SPEED, self.base.get_position()));
            self.money -= cost;
        }
    }

    pub fn remove_entity(&mut self, entity_to_remove : &entity::Fighter) {
        self.fighters.retain(|entity| entity != entity_to_remove);
    }

    pub fn decrease_life(&mut self, amount: i32) {
        self.base.set_health(self.base.get_health() - amount);
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
