use crate::story::Location;
// use super::locations::set_hero_location;
#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub hp: u32,
    pub armor: f64,
    pub attack: f64,
    pub exp: f64,
    pub lvl: u32,
    pub location: Location
}

impl Hero {

    pub fn set_location(&mut self, location: &Location) {
        self.location = location.clone();
    }

    pub fn print_details(&self) {
        println!("┌─────────────┬─────┬───────┬───────┬──────────────┐");
        println!("│    NAME     │ HP  │ ARMOR │ ATTACK│ EXPERIENCE   │");
        println!("├─────────────┼─────┼───────┼───────┼──────────────┤");
        println!("│ {:<12}│ {:<4}│ {:<6}│ {:<6}│ {:<12} │", &self.name, self.hp, self.armor, self.attack, self.exp);
        println!("└─────────────┴─────┴───────┴───────┴──────────────┘");
        println!("LEVEL: {}", self.lvl);
        println!("Currently at: {:?}", self.location);
    }
    
}

pub fn create_hero() -> Hero {
    let hero = Hero {
        name: String::from("Marc"),
        hp: 100,
        armor: 0.0,
        attack: 5.0,
        exp: 0.0,
        lvl: 1,
        location: Location::Home
    };
    return hero
}


// pub struct Enemy {
//         name: String,
//         hp: u32,
//         armor: f64,
//         attack: f64
//     }

// impl Enemy {
//     pub fn print_details(&self) {
//         println!("NAME: {} - HP: {}", &self.name, self.hp);
//         println!("ARMOR: {}", self.armor);
//         println!("ATTACK: {}", self.attack);
//     }
// }

// pub fn create_enemy() -> Enemy {
//     let enemy: Enemy = Enemy {
//         name: String::from("Skeleton"),
//         hp: 100,
//         armor: 0.0,
//         attack: 5.0
//     };
//     return enemy
// }