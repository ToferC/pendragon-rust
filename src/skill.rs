#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub value: u32,
}

impl Default for Skill {
    fn default() -> Skill {
        Skill {
            name: String::from("Sword"),
            value: 15,
        }
    }
}