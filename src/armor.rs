#[derive(Debug)]
pub struct Armor {
    name: String,
    value: u32,
}

impl Default for Armor {
    fn default() -> Armor {
        Armor {
            name: String::from("Chainmail"),
            value: 12,
        }
    }
}

#[derive(Debug)]
pub struct Shield {
    name: String,
    value: u32,
}

impl Default for Shield {
    fn default() -> Shield {
        Shield {
            name: String::from("Kite"),
            value: 12,
        }
    }
}
