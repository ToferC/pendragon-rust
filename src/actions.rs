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

pub struct CompareRolls {
    a: RollResult,
    b: RollResult,
}

#[derive(Debug)]
pub enum RollResult {
    Fumble ( u32 ),
    Failure ( u32 ),
    Tie ( u32 ),
    Potential ( u32 ),
    Success ( u32 ),
    Critical ( u32 ),
}

#[derive(Debug)]
pub enum OpposedResult {
    AWins( RollResult ),
    BWins( RollResult ),
    Tie( RollResult ),
}

pub fn get_skill_value_or_default(c: &Character, skill: &String) -> u32 {
    let skill_value = c.skills.get(skill);

    if let Some(i) = skill_value {
        *i
    } else {
        1
    }
}

pub fn evaluate_roll(mut roll: u32, mut value: u32) -> RollResult {

    // If value > 20, add +1 to roll for each point the skill exceeds 20 & set value to 20
    if value > 20 {
        roll += value - 20;
        value = 20;
    }

    // If total roll > 20, set to 20
    if roll > 20 {
        roll = 20;
    }

    // Match for roll
    match roll {
        // Roll == value -> Crit
        r if r == value => {
            println!("Critical!");
            RollResult::Critical( roll )
        },
        // Roll == 20 and value != 20 -> Fumble
        20 => {
            println!("Fumble!");
            RollResult::Fumble( roll )
        },
        // Potential Special Result based on skill/weapon
        1 => {
            println!("Potential Special");
            RollResult::Potential( roll )
        },
        // Roll < value -> Success
        r if r < value => {
            println!("Success");
            RollResult::Success( roll )
        },
        // Roll > value -> Failure
        _ => {
            println!("Failure");
            RollResult::Failure( roll )
        },
    }
}

pub fn resolution_phase(c: &Character, weapon: &String, target: &String) -> RollResult {

    println!{"{} attacks {} with a {}", c.name, target, c.weapon.name};

    let roll = roll_em(1, 20, 0);

    println!("{} rolls a {}", c.name, roll);

    let v = get_skill_value_or_default(c, weapon);

    evaluate_roll(roll, v)

}

pub fn unopposed_roll(name: &str, skill: &str, value: u32, mods: u32) -> RollResult {

    println!{"{} uses {} with a skill of {}", name, skill, value};

    let roll = roll_em(1, 20, mods);

    evaluate_roll(roll, value)
}

pub fn opposed_roll(a: &Character, a_skill: &String, a_mods: u32, b: &Character, b_skill: &String, b_mods: u32) -> OpposedResult {

    
    let a_val = get_skill_value_or_default(a, a_skill);
    println!{"{} uses {} with a skill of {}", a.name, a_skill, a_val};

    let a_result = evaluate_roll(roll_em(1, 20, a_mods), a_val);
    
    let b_val = get_skill_value_or_default(b, b_skill);
    println!{"{} uses {} with a skill of {}", b.name, b_skill, b_val};

    let b_result = evaluate_roll(roll_em(1, 20, b_mods), b_val);

    let cp = CompareRolls {
        a: a_result,
        b: b_result,
    }; 


    // Comparison Matrix
    // A crit & B crit == tie
    // A crit & B !crit == A crit
    // A !crit & B crit == B crit
    // A success & B success & results the same == tie
    // A success & B success & A result > B result == A success
    // A success & B success & A result < B result == B success
    // A success & B Failure || Fumble == A success
    // A Failure || Fumble & B Success == B success
    // A Failure & B Failure == tie


    match cp {
        CompareRolls {
            a: RollResult::Critical( a_r ), 
            b: RollResult::Critical( _ ) } => {
                println!("Critical Tie");
                OpposedResult::Tie( RollResult::Tie ( a_r ))
            },

        CompareRolls {
            a: RollResult::Critical( a_r ), b: _ } => {
                println!("{} Critical", &a.name);
                OpposedResult::AWins( RollResult::Critical ( a_r ))
            },

        CompareRolls {
            a: _, b: RollResult::Critical( b_r ) } => {
                println!("{} Critical", &b.name);
                OpposedResult::BWins( RollResult::Critical ( b_r ))
            },

        CompareRolls {
            a: RollResult::Success( a_r ), 
            b: RollResult::Success( b_r )} => {
                if a_r == b_r {
                    println!("Tie");
                    OpposedResult::Tie( RollResult::Critical ( a_r ))
                } else if a_r > b_r {
                    println!("{} Wins", &a.name);
                    OpposedResult::AWins( RollResult::Success( a_r ))
                } else {
                    println!("{} Wins", &a.name);
                    OpposedResult::BWins( RollResult::Success( b_r ))
                }
            },

        CompareRolls {
            a: RollResult::Success( a_r ), 
            b: _ } => {
                println!("{} Wins", &a.name);
                OpposedResult::AWins( RollResult::Success ( a_r ))
            },

        CompareRolls {
            a: _, 
            b: RollResult::Success( b_r ) } => {
                println!("{} Wins", &b.name);
                OpposedResult::BWins( RollResult::Success ( b_r ))
            },

        _ => {
            println!("Tie");
            OpposedResult::Tie( RollResult::Tie ( 1 ))
        },
    }

}