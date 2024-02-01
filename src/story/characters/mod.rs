use crate::utils;
use std::cmp;
use std::io;


#[derive(Debug)]
pub struct CharacterCommon {
    pub name: String,
    pub hp: u32,
    pub attack: f64,
    pub armor: f64,
}

#[derive(Debug)]
pub struct Hero {
    pub common: CharacterCommon,
    pub exp: f64,
    pub lvl: u32
}

impl Hero {
    pub fn new(name: String, hp: u32, attack: f64, armor: f64, exp: f64, lvl: u32) -> Self {
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
    
    pub fn take_damage(&mut self, damage: f64) {
        let actual_damage = damage * (1.0 - self.common.armor / 100.0);
        self.common.hp = cmp::max(0, self.common.hp as i32 - actual_damage as i32) as u32;
    }
}


pub struct Enemy {
        pub common: CharacterCommon,
    }

impl Enemy {
    pub fn new(name: String, hp: u32, attack: f64, armor: f64) -> Self {
        Enemy {
            common: CharacterCommon {name, hp, attack, armor}
        }
    }

    pub fn print_details(&self) {
        utils::print_character_details(&self.common.name, self.common.hp, self.common.armor, self.common.attack);
    }

    pub fn take_damage(&mut self, damage: f64) {
        let actual_damage = damage * (1.0 - self.common.armor / 100.0);
        self.common.hp = cmp::max(0, self.common.hp as i32 - actual_damage as i32) as u32;
    }
}

fn update_battle_screen(hero: &mut Hero, enemy: &mut Enemy) {
    utils::cls();
    utils::print_fight_details(&hero.common.name, 
        hero.common.hp, 
        hero.common.armor, 
        hero.common.attack, 
        &enemy.common.name, 
        enemy.common.hp, 
        enemy.common.armor, 
        enemy.common.attack);
}

fn battle_user_decision() -> String {
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

fn hero_attacks_enemy(hero: &mut Hero, enemy: &mut Enemy) -> u8 {
    // Hero attacks enemy
    let hero_damage = hero.common.attack;
    enemy.take_damage(hero_damage);
    update_battle_screen(hero, enemy);
    println!("Hero attacks Enemy for {} damage!", hero_damage);

    if enemy.common.hp == 0 {
        println!("Enemy defeated!");
        return 0;
    }
    utils::slp(1);
    // Enemy attacks hero
    let enemy_damage = enemy.common.attack;
    hero.take_damage(enemy_damage);
    update_battle_screen(hero, enemy);
    println!("Enemy attacks Hero for {} damage!", enemy_damage);

    if hero.common.hp == 0 {
        println!("Hero defeated!");
        return 1;
    }
    utils::slp(1);
    println!("--------------------------------");
    10
}

pub fn simulate_fight(hero: &mut Hero, enemy: &mut Enemy) {
    println!("Battle Start!");
    println!("Hero: {} vs Enemy: {}", hero.common.name, enemy.common.name);
    println!("--------------------------------");

    while hero.common.hp > 0 && enemy.common.hp > 0 {
        utils::cls();
        update_battle_screen(hero, enemy);
        println!("");
        let user_decision = battle_user_decision();

        // Fight
        if user_decision == "1" {
            let attack_outcome = hero_attacks_enemy(hero, enemy);
            if attack_outcome == 0 {
                println!("Hero wins the battle");
                update_battle_screen(hero, enemy);
                break;
            } else if attack_outcome == 1 {
                println!("Hero losses the battle");
                update_battle_screen(hero, enemy);
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
