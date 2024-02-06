use crate::story::characters::enemy::Enemy;

#[derive(Debug)]
pub struct Location {
    name: String,
    exp_boost: u32,
    enemies: Vec<Enemy>
}

impl Location {
    pub fn new(name: String, exp_boost: u32, enemies: Vec<Enemy>) -> Location {
        Location {name, exp_boost, enemies}
    }
}