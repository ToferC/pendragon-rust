#[derive(Debug)]
pub enum Homeland {
    Salisbury,
}

#[derive(Debug)]
pub struct Culture {
    name: String,
    modifier: String,
    value: u32,
}

impl Default for Culture {
    fn default() -> Culture {
        Culture {
            name: String::from("Cymric"),
            modifier: String::from("CON"),
            value: 3,
        }
    }
}

#[derive(Debug)]
pub struct Religion {
    name: String,
    virtues: Vec<String>,
}

impl Default for Religion {
    fn default() -> Religion {
        Religion {
            name: String::from("Christian"),
            virtues: vec!{"Chaste".to_string(),
                         "Forgiving".to_string(),
                         "Merciful".to_string(),
                         "Modest".to_string(),
                         "Temperate".to_string()}
                        }
    }
}