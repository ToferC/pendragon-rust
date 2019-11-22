use rand::Rng;

pub fn roll_em(dice: u32, sides: u32, mods: u32) -> u32 {

    let mut rng = rand::thread_rng();

    let mut total: u32 = 0;

    for _ in 1..dice+1 {
        total += rng.gen_range(1, sides + 1)
    };

    total += mods;

    println!("Roll {}d{}+{}: result of {}", dice, sides, mods, &total);

    total
}