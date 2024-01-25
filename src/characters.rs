#[derive(Debug)]
enum Location {
    Home,
    Woods,
    Mountains,
    Riverside,
    Ocean
}

pub struct Hero {
    name: String,
    hp: u32,
    armor: f64,
    attack: f64,
    exp: f64,
    location: Location
}

impl Hero {
    pub fn print_details(&self) {
        println!("NAME: {} - HP: {} - ARMOR: {}", &self.name, self.hp, self.armor);
        println!("ARMOR: {} - ATTACK: {}", self.armor, self.attack);
        println!("EXPERIENCE: {}", self.exp);
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
        location: Location::Home
    };
    return hero
}

pub struct Enemy {
    name: String,
    hp: u32,
    armor: f64,
    attack: f64
}

impl Enemy {
    pub fn print_details(&self) {
        println!("NAME: {} - HP: {}", &self.name, self.hp);
        println!("ARMOR: {}", self.armor);
        println!("ATTACK: {}", self.attack);
    }
}

pub fn create_enemy() -> Enemy {
    let enemy: Enemy = Enemy {
        name: String::from("Skeleton"),
        hp: 100,
        armor: 0.0,
        attack: 5.0
    };
    return enemy
}