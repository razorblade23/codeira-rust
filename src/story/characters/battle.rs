use crate::utils;
use std::io;

use super::{enemy::Enemy, hero::Hero};

pub struct Battle<'a> {
    hero: &'a mut Hero,
    enemy: &'a mut Enemy
}

impl<'a> Battle<'a> {
    pub fn new(hero: &'a mut Hero, enemy: &'a mut Enemy) -> Battle<'a> {
        Battle { hero, enemy }
    }

    fn update_battle_screen(&self) {
        utils::cls();
        utils::print_fight_details(
            &self.hero.common.name, 
            self.hero.common.hp, 
            self.hero.common.armor, 
            self.hero.common.attack, 
            &self.enemy.common.name, 
            self.enemy.common.hp, 
            self.enemy.common.armor, 
            self.enemy.common.attack);
    }
    
    fn battle_menu(&self) -> String {
        let menu = [
            ("1", "Fight"),
            ("2", "Change weapon (not implemented)"),
            ("3", "Drink potion (not implemented)"),
            ("4", "Run from battle (not implemented)"),
        ];
        utils::print_menu(menu);
        println!("");
        println!("Select action:");
        let mut path = String::new();
    
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read line");
    
        let user_choice = path.trim();
        return user_choice.to_string()
    }
    
    fn hero_attacks_enemy(&mut self) -> u8 {
        // Hero attacks enemy
        let hero_damage = self.hero.common.attack;
        self.enemy.take_damage(hero_damage);
        self.update_battle_screen();
        println!("Hero attacks Enemy for {} damage!", hero_damage);
    
        if self.enemy.common.hp == 0 {
            println!("Enemy defeated!");
            return 0;
        }
        utils::slp(1);
        // Enemy attacks hero
        let enemy_damage = self.enemy.common.attack;
        self.hero.take_damage(enemy_damage);
        println!("Enemy attacks Hero for {} damage!", enemy_damage);
        self.update_battle_screen();
    
        if self.hero.common.hp == 0 {
            println!("Hero defeated!");
            return 1;
        }
        utils::slp(1);
        println!("--------------------------------");
        10
    }
    
    pub fn simulate_fight(&mut self) {
        while self.hero.common.hp > 0 && self.enemy.common.hp > 0 {
            utils::cls();
            self.update_battle_screen();
            println!("");
            let user_decision = self.battle_menu();
    
            // Fight
            if user_decision == "1" {
                let battle_outcome = self.hero_attacks_enemy();
                if battle_outcome == 0 {
                    println!("Hero wins the battle");
                    self.update_battle_screen();
                    break;
                } else if battle_outcome == 1 {
                    println!("Hero losses the battle");
                    self.update_battle_screen();
                    break;
                }
            // Change weapon
            } else if user_decision == "2" {
                println!("Not implemented ...")
            // Drink potion
            } else if user_decision == "3" {
                println!("Not implemented ...")
            // Run
            } else if user_decision == "4" {
                println!("Not implemented ...")
            }
        }
        println!("Battle End!");
    }
}
