use std::thread;
use std::time::Duration;

// use crate::story::locations::Location;

pub fn slp(time_to_sleep: u64) {
    thread::sleep(Duration::from_secs(time_to_sleep));
}

pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

// pub fn parse_location(location: String) -> Location {
//     if location == "1" {
//         println!("Hero chosen WOODS to explore");
//         return Location::Woods{exp_boost: 10}
//     } else if  location == "2" {
//         println!("Hero chosen MOUNTAINS to explore");
//         return Location::Mountains{exp_boost: 20}
//     } else if location == "3" {
//         println!("Hero chosen RIVERSIDE to explore");
//         return Location::Riverside{exp_boost: 30}
//     } else if location == "4" {
//         println!("Hero chosen OCEAN to explore");
//         return Location::Ocean{exp_boost: 40}
//     } else {
//         println!("Not a valid input");
//         return Location::Home
//     }
// }

pub fn print_menu(menu: [(&str, &str); 4]) {
    for (key, value) in &menu {
        println!("{}) {}", key, value);
    }
}

pub fn print_character_details(name: &String, hp: u32, armor: u32, attack: u32) {
    println!("┌─────────────┬─────┬───────┬────────┐");
    println!("│    NAME     │ HP  │ ARMOR │ ATTACK │");
    println!("├─────────────┼─────┼───────┼────────┤");
    println!("│ {:<12}│ {:<4}│ {:<6}│ {:<6} │", name, hp, armor, attack);
    println!("└─────────────┴─────┴───────┴────────┘");
}

pub fn print_fight_details(hero_name: &String, hero_hp: u32, hero_armor: u32, hero_attack: u32, 
    enemy_name: &String, enemy_hp: u32, enemy_armor: u32, enemy_attack: u32) {
    println!("┌─────────────┬─────┬───────┬────────┐       ┌─────────────┬─────┬───────┬────────┐");
    println!("│    HERO     │ HP  │ ARMOR │ ATTACK │       │   ENEMY     │ HP  │ ARMOR │ ATTACK │");
    println!("├─────────────┼─────┼───────┼────────┤  VS   ├─────────────┼─────┼───────┼────────┤");
    println!("│ {:<12}│ {:<4}│ {:<6}│ {:<6} │       │ {:<12}│ {:<4}│ {:<6}│ {:<6} │", 
    hero_name, hero_hp, hero_armor, hero_attack, enemy_name, enemy_hp, enemy_armor, enemy_attack);
    println!("└─────────────┴─────┴───────┴────────┘       └─────────────┴─────┴───────┴────────┘");
}