use pendragon::character::{Character, load_character};
use pendragon::combat::*;
use pendragon::actions::{unopposed_roll};

fn main() -> std::io::Result<()> {

    let mut combat = Combat::new();

    // First Knight
    let mut c = Character::default();

    &c.rename();

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

    *g.skills.get_mut("Sword").unwrap() += 15;
    g.strength = 15;
    g.constitution = 15;
    g.size = 15;

    println!("\nPrinting Character\n");

    println!("{:#?}\n", g);

    // Skill roll - Unopposed

    let s1 = unopposed_roll(&g.name, "Sword", g.skills["Sword"], 0);

    println!("{:#?}\n", s1);
    
    // Second Knight
    let mut d = Character::default();

    d.name = String::from("Brigand One");

    combat.add_fighter(&mut g);
    combat.add_fighter(&mut d);

    combat.start();

    Ok(())
    
}
