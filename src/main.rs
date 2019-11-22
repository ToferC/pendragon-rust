use pendragon::character::Character;
use pendragon::combat::combat;

use std::fs::File;
use std::io::{prelude::*, stdin};


fn load_character(n: &String) -> std::io::Result<String> {
    let mut file = File::open(format!("{}.txt", n))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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

    let output = match result {
        Ok(output) => output,
        Err(e) => return Err(e),
    };

    println!("\nPrinting Character\n");

    println!("{}", output);

    // Second Knight
    let mut d = Character::default();

    name = String::new();

    println!("Enter the second knight's name: ");
    stdin().read_line(&mut name)?;

    d.name = name.trim_end_matches("\n").to_string();

    println!("\nSaving Character\n");
    d.save()?;

    combat(&mut c, &mut d);

    Ok(())
    
}
