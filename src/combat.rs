use crate::character::{Character, CharacterState};
use crate::actions::*;
use crate::rules::*;

// use crate::actions::attack;

pub fn combat<'a>(a: &'a mut Character, b: &'a mut Character) {
    
    // Update combat to have active state & match for combatant states

    println!("Attacking!");

    while a.state != CharacterState::Dead && b.state != CharacterState::Dead {

        let opposed = opposed_roll(a, &"Sword".to_string(), 0, b, &"Sword".to_string(), 0);
        println!("{:#?}", opposed);

        match opposed {
            OpposedResult::AWins( RollResult::Critical ( _ )) => {
                let dam = roll_em(a.damage, 6, 0) * 2;
                if dam < b.armor.reduction {
                    println!{"Armor protects!"};
                } else {
                    b.harm(dam - b.armor.reduction);
                }
            },

            OpposedResult::BWins( RollResult::Critical ( _ )) => {
                let dam = roll_em(b.damage, 6, 0) * 2;
                 if dam < a.armor.reduction {
                    println!{"Armor protects!"};
                 } else {
                     a.harm(dam - a.armor.reduction);
                 }
            },

            OpposedResult::AWins( RollResult::PartialCritical ( _ )) => {
                let dam = roll_em(a.damage, 6, 0) * 2;
                if dam < b.armor.reduction + b.shield.value {
                    println!{"Armor protects!"};
                 } else {
                     b.harm(dam - b.armor.reduction - b.shield.value);
                 }
            },

            OpposedResult::BWins( RollResult::PartialCritical ( _ )) => {
                let dam = roll_em(b.damage, 6, 0) * 2;
                if dam < a.armor.reduction + a.shield.value {
                    println!{"Armor protects!"};
                 } else {
                     a.harm(dam - a.armor.reduction - a.shield.value);
                 }
            },

            OpposedResult::AWins( RollResult::Success ( _ )) => {
                let dam = roll_em(a.damage, 6, 0);
                if dam < b.armor.reduction {
                    println!{"Armor protects!"};
                 } else {
                     b.harm(dam - b.armor.reduction);
                 }
            },

            OpposedResult::BWins( RollResult::Success ( _ )) => {
                let dam = roll_em(b.damage, 6, 0);
                if dam < a.armor.reduction {
                    println!{"Armor protects!"};
                 } else {
                     a.harm(dam - a.armor.reduction);
                 }
            },

            OpposedResult::AWins( RollResult::PartialSuccess ( _ )) => {
                let dam = roll_em(a.damage, 6, 0);
                if dam < b.armor.reduction + b.shield.value {
                    println!{"Armor protects!"};
                 } else {
                     b.harm(dam - b.armor.reduction - b.shield.value);
                 }
            },

            OpposedResult::BWins( RollResult::PartialSuccess ( _ )) => {
                let dam = roll_em(b.damage, 6, 0);
                if dam < a.armor.reduction + a.shield.value {
                    println!{"Armor protects!"};
                 } else {
                     a.harm(dam - a.armor.reduction - a.shield.value);
                 }
            },

            _ => (),
        }
        

    };
}