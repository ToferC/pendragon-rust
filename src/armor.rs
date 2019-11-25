use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Armor {
    pub name: String,
    pub reduction: u32,
    pub dex_modifier: i32,
    pub heavy_load: bool,
}

impl Default for Armor {
    fn default() -> Armor {
        Armor {
            name: String::from("Chainmail"),
            reduction: 10,
            dex_modifier: -10,
            heavy_load: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shield {
    pub name: String,
    pub value: u32,
}

impl Default for Shield {
    fn default() -> Shield {
        Shield {
            name: String::from("Shield"),
            value: 6,
        }
    }
}
