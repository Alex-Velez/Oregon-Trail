use std::collections::HashMap;

use crate::{difficulty::Difficulty, state::Materials};

pub struct Player {
    pub name: String,
    pub health: i32,
    pub food: i32,
    pub stamina: i32,
    pub moral: i32,
    pub backpack: HashMap<Materials, u32>,
    pub effects: Vec<Effects>,
    pub bleeding: bool,
}

impl Player {
    pub fn new(name: String, difficulty: Difficulty) -> Player {
        Player {
            name,
            health: difficulty.start_health,
            food: difficulty.start_food,
            stamina: difficulty.start_stamina,
            moral: difficulty.start_moral,
            backpack: HashMap::new(),
            effects: Vec::new(),
            bleeding: false,
        }
    }
}

pub enum Effects {}
