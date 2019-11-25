use pendragon::character::{Character, load_character};
use pendragon::combat::*;
use pendragon::actions::{unopposed_roll};

use std::io::stdin;

fn main() -> std::io::Result<()> {

    // First Knight
    let mut c = Character::default();

    let mut name = String::new();
    println!("Enter the first knight's name: ");
    stdin().read_line(&mut name)?;

    c.name = name.trim_end_matches("\n").to_string();

    // Save character
    println!("\nSaving Character\n");
    c.save()?;

    // Load character
    println!("\nLoading Character\n");
    let result = load_character(&c.name);

    let mut g = match result {
        Ok(g) => g,
        Err(e) => return Err(e),
    };

    println!("\nPrinting Character\n");

    println!("{:#?}\n", g);

    // Skill roll - Unopposed

    let s1 = unopposed_roll(&g.name, "Sword", g.skills["Sword"], 0);

    println!("{:#?}\n", s1);
    
    // Second Knight
    let mut d = Character::default();

    name = String::new();

    println!("Enter the second knight's name: ");
    stdin().read_line(&mut name)?;

    d.name = name.trim_end_matches("\n").to_string();

    println!("\nSaving Character\n");
    d.save()?;

    combat(&mut g, &mut d);

    Ok(())
    
}
