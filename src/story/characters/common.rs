
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
pub struct CharacterCommon {
    pub name: String,
    pub hp: u32,
    pub attack: u32,
    pub armor: u32,
}