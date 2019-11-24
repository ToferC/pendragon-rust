use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub damage: u32,
}

impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            name: String::from("Sword"),
            damage: 0,
        }
    }
}