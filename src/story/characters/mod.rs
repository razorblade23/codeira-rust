mod common;
pub mod hero;
pub mod enemy;
pub mod battle;

use rand::{thread_rng, Rng};
use self::{hero::Hero, enemy::Enemy};

pub fn generate_enemies(hero: &Hero) -> Vec<Enemy> {
    let mut rng = thread_rng();

    let enemy_names = ["Goblin", "Skeleton", "Orc", "Troll", "Dragon"];
    let num_enemies = rng.gen_range(3..16);

    let mut enemy_list = Vec::new();

    for _ in 0..num_enemies {
        let name_seed = rng.gen_range(0..enemy_names.len());
            let name = enemy_names[name_seed];

            let enemy_hp = rng.gen_range(1..hero.common.hp);

            let enemy_attack = rng.gen_range(1..hero.common.attack);
            let enemy_armor = rng.gen_range(1..hero.common.armor);

            let enemy = Enemy::new(name.to_string(), enemy_hp, enemy_attack, enemy_armor);
            enemy_list.push(enemy);
    }
    enemy_list
}