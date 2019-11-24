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

#[derive(Debug)]
pub enum RollResult {
    Fumble { result: u32 },
    Failure { result: u32 },
    Success { result: u32 },
    Critical { result: u32 },
}

pub fn attack(c: &Character, target: &Character) -> Attack {

    println!{"{} attacks {} with a {}", c.name, target.name, c.weapon.name};

    let roll = roll_em(1, 20, 0);

    println!("{} rolls a {}", c.name, roll);

    match roll {
        r if r + c.dexterity < (target.dexterity + 10) => Attack { attack_result: AttackResult::Miss, damage: 0 },
        r if r + c.dexterity >= (target.dexterity + 15) => Attack { attack_result: AttackResult::Critical, damage: roll_em(c.damage, 6, 5) },
        r if r + c.dexterity >= (target.dexterity + 10) => Attack { attack_result: AttackResult::Hit, damage: roll_em(c.damage, 6, 0) },
        _ => Attack { attack_result: AttackResult::Miss, damage: 0 },
    }
}

pub fn unopposed_roll(name: &str, skill: &str, value: u32, mods: u32) -> RollResult {

    println!{"{} uses {} with a skill of {}", name, skill, value};

    let roll = roll_em(1, 20, mods);

    match roll {
        value => RollResult::Critical{ result: roll },
        20 => RollResult::Fumble{ result: roll },
        r if r < value => RollResult::Success{ result: roll },
        _ => RollResult::Failure{ result: roll },
    }
}