use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub skill_type: SkillType,
    pub value: u32,
    pub experience_check: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillType {
    Standard,
    Combat,
}

impl Default for Skill {
    fn default() -> Skill {
        Skill {
            name: String::from(""),
            skill_type: SkillType::Standard,
            value: 15,
            experience_check: false,
        }
    }
}