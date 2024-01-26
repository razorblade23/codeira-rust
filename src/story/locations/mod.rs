#[derive(Clone)]
#[derive(Debug)]
pub enum Location {
    Home ,
    Woods{exp_boost: u32},
    Mountains{exp_boost: u32},
    Riverside{exp_boost: u32},
    Ocean{exp_boost: u32}
}