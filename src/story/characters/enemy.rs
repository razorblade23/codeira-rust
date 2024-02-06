use crate::story::characters::common::CharacterCommon;
use crate::utils;
use std::cmp;


#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct Enemy {
        pub common: CharacterCommon,
    }

impl Enemy {
    pub fn new(name: String, hp: u32, attack: u32, armor: u32) -> Self {
        Enemy {
            common: CharacterCommon {name, hp, attack, armor}
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        let actual_damage = damage * (1 - self.common.armor / 100);
        self.common.hp = cmp::max(0, self.common.hp as i32 - actual_damage as i32) as u32;
    }
}