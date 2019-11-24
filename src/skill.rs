use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub skill_type: SkillType,
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillType {
    Standard,
    Combat,
}

impl Default for Skill {
    fn default() -> Skill {
        Skill {
            name: String::from("Sword"),
            skill_type: SkillType::Combat,
            value: 15,
        }
    }
}