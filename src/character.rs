use std::collections;
use std::fs::File;
use std::io::{Write, BufReader};

use serde::{Serialize, Deserialize};
use serde_json;

use crate::rules::roll_em;
use crate::weapon;
use crate::armor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    // Represents a character in Pendragon
    pub name: String,
    pub glory: u32,
    pub homeland: String,
    pub culture: String,
    pub religion: String,
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
    pub features: Vec<String>,
    pub weapon: weapon::Weapon,
    pub armor: armor::Armor,
    pub shield: armor::Shield,
    pub damage: u32,
    healing_rate: u32,
    movement_rate: u32,
    total_hit_points: i32,
    hit_points: i32,
    unconscious: i32,
    pub state: CharacterState,
    pub distinctive_features: Vec<String>,
    pub skills: collections::HashMap<String, u32>,
}

impl Default for Character {
    fn default() -> Character {
        let mut c = Character {
            name: String::from(""),
            glory: roll_em(6, 6, 0) + 150,
            homeland: String::from("Salisbury"),
            culture: String::from("Cymric"),
            religion: String::from("Christian"),
            titles: String::from("Sir"),
            home: String::from("Gloucester"),
            age: 21,
            year_born: 468,
            traits: Vec::new(),
            passions: Vec::new(),
            size: roll_em(3, 6, 0),
            dexterity: roll_em(3, 6, 0),
            strength: roll_em(3, 6, 0),
            constitution: roll_em(3, 6, 0),
            appearance: roll_em(3, 6, 0),
            features: Vec::new(),
            weapon: weapon::Weapon::default(),
            armor: armor::Armor::default(),
            shield: armor::Shield::default(),
            damage: 1,
            healing_rate: 1,
            movement_rate: 1,
            hit_points: 1,
            total_hit_points: 1,
            unconscious: 1,
            state: CharacterState::Up,
            distinctive_features: Vec::new(),
            skills: collections::HashMap::new(),
        };

        // Add Culture Modifier
        c.constitution += 3;

        c.damage = (c.size + c.strength) / 6;
        c.healing_rate = (c.constitution + c.strength) / 10;
        c.movement_rate = (c.strength + c.dexterity) / 10;
        c.total_hit_points = c.constitution as i32 + c.size as i32;
        c.unconscious = c.total_hit_points / 4;

        c.hit_points = c.total_hit_points;

        // Add features

        if c.appearance > 12 {
            c.features.push("Long Hair".to_string());
        }

        // Add base skills

        c.skills.insert("Awareness".to_string(), 5);
        c.skills.insert("Boating".to_string(), 1);
        c.skills.insert("Compose".to_string(), 1);
        c.skills.insert("Courtesy".to_string(), 3);
        c.skills.insert("Dancing".to_string(), 2);
        c.skills.insert("Faerie Lore".to_string(), 1);
        c.skills.insert("Falconry".to_string(), 3);
        c.skills.insert("First Aid".to_string(), 10);
        c.skills.insert("Flirting".to_string(), 3);
        c.skills.insert("Folk Lore".to_string(), 2);
        c.skills.insert("Gaming".to_string(), 3);
        c.skills.insert("Heraldry".to_string(), 3);
        c.skills.insert("Hunting".to_string(), 2);
        c.skills.insert("Intrigue".to_string(), 3);
        c.skills.insert("Orate".to_string(), 3);
        c.skills.insert("Play (Harp)".to_string(), 3);
        c.skills.insert("Read (Latin)".to_string(), 0);
        c.skills.insert("Recognize".to_string(), 3);
        c.skills.insert("Religion (Christianity)".to_string(), 2);
        c.skills.insert("Romance".to_string(), 2);
        c.skills.insert("Singing".to_string(), 2);
        c.skills.insert("Stewardship".to_string(), 2);
        c.skills.insert("Swimming".to_string(), 2);
        c.skills.insert("Tourney".to_string(), 2);
        c.skills.insert("Battle".to_string(), 10);
        c.skills.insert("Horsemanship".to_string(), 10);
        c.skills.insert("Sword".to_string(), 10);
        c.skills.insert("Lance".to_string(), 10);
        c.skills.insert("Spear".to_string(), 6);
        c.skills.insert("Dagger".to_string(), 5);

        // Character improvement
        *c.skills.get_mut("Sword").unwrap() += 5;

        // Return Character

        c
    }
}

impl Character {
    pub fn harm(&mut self, damage: u32) {

        println!("{} is hit for {} damage!", self.name, damage);
        self.hit_points -= damage as i32;
        match self.hit_points {
            h if h < 0 => self.state = CharacterState::Dead,
            h if h <= self.unconscious => self.state = CharacterState::Unconscious,
            _ => (),
        }
        
        println!("{} has {} hit points left!", self.name, self.hit_points);
        
        match self.state {
            CharacterState::Unconscious => println!("{} is out!", self.name),
            CharacterState::Dead => println!("{} is dead!", self.name),
            _ => ()
        } 
    }

    pub fn heal(&mut self, healing: i32) {
        self.hit_points += healing;
        if self.hit_points >= self.unconscious {
            self.state = CharacterState::Up;
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
    
        let mut file = File::create(format!("./knights/{}.json", self.name))?;
        file.write_all(&serialized.into_bytes())?;
        Ok(())
    }
}

pub fn load_character(n: &String) -> std::io::Result<Character> {
    let file = File::open(format!("./knights/{}.json", n))?;
    let reader = BufReader::new(file);
    let character = serde_json::from_reader(reader)?;
    
    Ok(character)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trait {
    name: String,
    value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Passion {
    name: String,
    value: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CharacterState {
    Up,
    Unconscious,
    Dead,
    Weary,
    Mad,
}