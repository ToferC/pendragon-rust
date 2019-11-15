use rand::Rng;
use std::collections;

#[derive(Debug)]
pub struct Character {
    // Represents a character in Pendragon
    pub name: String,
    pub homeland: Homeland,
    pub culture: Culture,
    pub religion: Religion,
    pub titles: String,
    pub home: String,
    pub age: u32,
    pub year_born: u32,
    pub traits: Vec<Trait>,
    pub passions: Vec<Passion>,
    pub size: u32,
    pub dexterity: u32,
    pub strength: u32,
    pub constitution: u32,
    pub appearance: u32,
    damage: u32,
    healing_rate: u32,
    movement_rate: u32,
    total_hit_points: u32,
    unconscious: u32,
    pub distinctive_features: Vec<String>,
    pub skills: collections::HashMap<String, u32>,
}

impl Default for Character {
    fn default() -> Character {
        let mut c = Character {
            name: String::from(""),
            homeland: Homeland::Salisbury,
            culture: Culture {
                name: String::from("Cymric"),
                modifier: String::from("CON"),
                value: 3,
            },
            religion: Religion {
                name: String::from("Christian"),
                virtues: vec!{"Chaste".to_string(),
                             "Forgiving".to_string(),
                             "Merciful".to_string(),
                             "Modest".to_string(),
                             "Temperate".to_string()}
                            },
            titles: String::from(""),
            home: String::from(""),
            age: 21,
            year_born: 468,
            traits: Vec::new(),
            passions: Vec::new(),
            size: roll_em(3, 6, 0),
            dexterity: roll_em(3, 6, 0),
            strength: roll_em(3, 6, 0),
            constitution: roll_em(3, 6, 0),
            appearance: roll_em(3, 6, 0),
            damage: 1,
            healing_rate: 1,
            movement_rate: 1,
            total_hit_points: 1,
            unconscious: 1,
            distinctive_features: Vec::new(),
            skills: collections::HashMap::new(),
        };

        // Add Culture Modifier
        c.constitution += 3;

        c.damage = (c.size + c.strength) / 6;
        c.healing_rate = (c.constitution + c.strength) / 10;
        c.movement_rate = (c.strength + c.dexterity) / 10;
        c.total_hit_points = c.constitution + c.size;
        c.unconscious = c.total_hit_points / 4;

        c

    }
}

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

#[derive(Debug)]
pub struct Religion {
    name: String,
    virtues: Vec<String>,
}

#[derive(Debug)]
pub struct Trait {
    name: String,
    value: u32,
}

#[derive(Debug)]
pub struct Passion {
    name: String,
    value: u32,
}

pub fn roll_em(dice: u32, sides: u32, mods: u32) -> u32 {

    let mut rng = rand::thread_rng();

    let mut total: u32 = 0;

    for _ in 1..dice+1 {
        total += rng.gen_range(1, sides + 1)
    };

    total += mods;

    total
}