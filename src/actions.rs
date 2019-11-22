use crate::character::Character;
use crate::rules::roll_em;

#[derive(Debug)]
pub struct Attack {
    pub attack_result: AttackResult,
    pub damage: u32,
}

#[derive(Debug)]
pub enum AttackResult {
    Hit,
    Miss,
    Critical,
}

pub fn attack(c: &Character, target: &Character) -> Attack {

    println!{"{} attacks {} with a {}", c.name, target.name, c.weapon.name};

    let roll = roll_em(1, 20, 0);

    println!("{} rolls a {}", c.name, roll);

    match roll {
        r if r + c.dexterity < (target.dexterity + 10) => Attack { attack_result: AttackResult::Miss, damage: 0 },
        r if r + c.dexterity >= (target.dexterity + 15) => Attack { attack_result: AttackResult::Critical, damage: roll_em(c.weapon.damage, 6, 5) },
        r if r + c.dexterity >= (target.dexterity + 10) => Attack { attack_result: AttackResult::Hit, damage: roll_em(c.weapon.damage, 6, 0) },
        _ => Attack { attack_result: AttackResult::Miss, damage: 0 },
    }
}