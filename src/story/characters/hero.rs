use crate::story::characters::common::CharacterCommon;
use crate::utils;
use std::cmp;

#[derive(Debug)]
pub struct Hero {
    pub common: CharacterCommon,
    pub exp: f64,
    pub lvl: u32
}

impl Hero {
    pub fn new(name: String, hp: u32, attack: u32, armor: u32, exp: f64, lvl: u32) -> Self {
        Hero {
            common: CharacterCommon {name, hp, attack, armor},
            exp,
            lvl,
        }
    }

    pub fn print_details(&self) {
        utils::print_character_details(&self.common.name, self.common.hp, self.common.armor, self.common.attack);
        println!("EXPERIENCE: {}", self.exp);
        println!("LEVEL: {}", self.lvl);
    }
    
    pub fn take_damage(&mut self, damage: u32) {
        let actual_damage = damage * (1 - self.common.armor / 100);
        self.common.hp = cmp::max(0, self.common.hp as i32 - actual_damage as i32) as u32;
    }

}