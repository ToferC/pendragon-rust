use crate::character::{Character, CharacterState};
use crate::actions::*;
use crate::rules::*;
use crate::weapon::*;

pub struct Combat<'a> {
    fighters: Vec<&'a mut Character>,
    pub active: bool,
    pub round: u32,
}

impl<'a> Combat<'a> {
    pub fn new() -> Combat<'a> {
        Combat {
            fighters: Vec::new(),
            active: true,
            round: 1,
        }
    }

    pub fn add_fighter(&mut self, c: &'a mut Character) {
        self.fighters.push(c);
    }

    pub fn remove_fighter(&mut self, i: usize) {
        self.fighters.remove(i);
    }

    pub fn start(&mut self) {

        println!("Combat!");
        let b = self.fighters.pop().unwrap();

        let a = self.fighters.pop().unwrap();

        while self.active {


            println!("ROUND {}!\n----------------------\n", self.round);

            // Determination Phase

            let opposed = opposed_roll(a, &"Sword".to_string(), 0, b, &"Sword".to_string(), 0);
            println!("{}", opposed);

            // Resolution Phase

            resolve_round(opposed, a, b);

            // Check to see if combat continues
            if (a.state == CharacterState::Unconscious || a.state == CharacterState::Dead) || (b.state == CharacterState::Unconscious || b.state == CharacterState::Dead) {
                self.active = false;
                break;
            }

            self.round += 1;

        };

        println!("\nFIGHT LASTED {} ROUNDS", self.round);
    }
}

pub enum Action {
    ExchangeBlows ( Character ),
    Attack ( Character ),
    Dodge,
    Move,
    ReArm ( Weapon ),
}

pub enum Modifiers {
    Cover ( i32 ),
    Fatigue ( i32 ),
    HigherGround ( i32 ),
    Immobilized ( i32 ),
    Suprise ( i32 ),
    Unencumbered ( i32 ),
    Visibility ( i32 ),
}

fn resolve_round(op: OpposedResult, a: &mut Character, b: &mut Character) {
    
    match op {
        OpposedResult::AWins( RollResult::Critical ( _ )) => {
            let dam = roll_em(a.damage, 6, 0) * 2;
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", b.armor.reduction};
            if dam <= b.armor.reduction {
                println!("No damage!")
            } else {
                b.harm(dam - b.armor.reduction);
            }
        },

        OpposedResult::BWins( RollResult::Critical ( _ )) => {
            let dam = roll_em(b.damage, 6, 0) * 2;
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", a.armor.reduction};
             if dam <= a.armor.reduction {
                println!("No damage!")
             } else {
                 a.harm(dam - a.armor.reduction);
             }
        },

        OpposedResult::AWins( RollResult::PartialCritical ( _ )) => {
            let dam = roll_em(a.damage, 6, 0) * 2;
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", b.armor.reduction};
            println!("Shield reduces damage by {}!", b.shield.value);
            if dam <= (b.armor.reduction + b.shield.value) {
                println!("No damage!")
             } else {
                 b.harm(dam - b.armor.reduction - b.shield.value);
             }
        },

        OpposedResult::BWins( RollResult::PartialCritical ( _ )) => {
            let dam = roll_em(b.damage, 6, 0) * 2;
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", a.armor.reduction};
            println!("Shield reduces damage by {}!", a.shield.value);
            if dam <= (a.armor.reduction + a.shield.value) {
                println!("No damage!")
             } else {
                 a.harm(dam - a.armor.reduction - a.shield.value);
             }
        },

        OpposedResult::AWins( RollResult::Success ( _ )) => {
            let dam = roll_em(a.damage, 6, 0);
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", b.armor.reduction};
            if dam <= b.armor.reduction {
                println!("No damage!")
             } else {
                 b.harm(dam - b.armor.reduction);
             }
        },

        OpposedResult::BWins( RollResult::Success ( _ )) => {
            let dam = roll_em(b.damage, 6, 0);
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", a.armor.reduction};
            if dam <= a.armor.reduction {
                println!("No damage!")
             } else {
                 a.harm(dam - a.armor.reduction);
             }
        },

        OpposedResult::AWins( RollResult::PartialSuccess ( _ )) => {
            let dam = roll_em(a.damage, 6, 0);
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", b.armor.reduction};
            println!("Shield reduces damage by {}!", b.shield.value);
            if dam <= (b.armor.reduction + b.shield.value) {
                println!("No damage!")
             } else {
                 b.harm(dam - b.armor.reduction - b.shield.value);
             }
        },

        OpposedResult::BWins( RollResult::PartialSuccess ( _ )) => {
            let dam = roll_em(b.damage, 6, 0);
            println!("Hit for {} damage!", &dam);
            println!{"Armor reduces damage by {}!", a.armor.reduction};
            println!("Shield reduces damage by {}!", a.shield.value);
            if dam <= (a.armor.reduction + a.shield.value) {
                println!("No damage!")
             } else {
                 a.harm(dam - a.armor.reduction - a.shield.value);
             }
        },

        _ => (),
    }
        println!("---------------------------");
}