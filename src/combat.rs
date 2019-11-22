use crate::character::{Character, CharacterState};
use crate::actions::attack;

pub fn combat<'a>(c: &'a mut Character, target: &'a mut Character) {
    
    let mut combatants = vec!{c, target};

    println!("Attacking!");

    let mut counter = 0;

    while combatants[0].state != CharacterState::Unconscious && combatants[1].state != CharacterState::Unconscious {
        let t: usize;

        if counter == 0 {
            t = 1
        } else {
            t = 0
        };

        let a = attack(combatants[counter], combatants[t]);
        println!("{:?}", a);

        if a.damage > 0 {
            combatants[t].harm(a.damage as i32);
        }

        if counter == 0 {
            counter = 1
        } else {
            counter = 0
        };
    };
}