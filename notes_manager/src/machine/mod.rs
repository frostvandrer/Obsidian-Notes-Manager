use std::fmt;
use crate::lab::Lab;

#[derive(Debug)]
pub enum MachineDifficulty {
    Easy,
    Medium,
    Hard,
    Insane
}

impl fmt::Display for MachineDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MachineDifficulty::Easy => write!(f, "Easy"),
            MachineDifficulty::Medium => write!(f, "Medium"),
            MachineDifficulty::Hard => write!(f, "Hard"),
            MachineDifficulty::Insane => write!(f, "Insane"),
        }
    }
}

pub struct MachineInfo {
    pub name: String,
    pub lab: Lab,
    pub difficulty: MachineDifficulty,
    pub path: String
}

impl MachineInfo {
    pub fn new(name: String, lab: Lab, difficulty: MachineDifficulty, path: String) -> MachineInfo {
        MachineInfo {
            name,
            lab,
            difficulty,
            path
        }
    }
}
