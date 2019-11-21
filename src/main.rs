use pendragon::{
    Character,
};
use std::fs::File;
use std::io::{prelude::*, stdin, Write};

fn load_character(n: String) -> std::io::Result<String> {
    let mut file = File::open(format!("{}.txt", n))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let mut c = Character::default();

    let mut name = String::new();
    println!("Enter your character's name: ");
    stdin().read_line(&mut name)?;

    c.name = name.trim_end_matches("\n").to_string();

    println!("\nSaving Character\n");
    c.save()?;

    println!("\nLoading Character\n");
    let result = load_character(c.name);

    let output = match result {
        Ok(output) => output,
        Err(e) => return Err(e),
    };

    println!("\nPrinting Character\n");

    println!("{}", output);

    Ok(())
    
}
