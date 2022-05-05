use rand::{thread_rng, Rng};
use std::io;

enum Weapon {
    Unknown,
    Paper,
    Rock,
    Scissors,
}

impl Weapon {
    fn value(&self) -> u32 {
        match *self {
            Weapon::Unknown => 0,
            Weapon::Paper => 1,
            Weapon::Rock => 2,
            Weapon::Scissors => 3,
        }
    }

    fn matching(value: &u32) -> Weapon {
        match value {
            1 => Weapon::Paper,
            2 => Weapon::Rock,
            3 => Weapon::Scissors,
            _ => Weapon::Unknown,
        }
    }
}

pub fn Gamble() {
    println!("Welcome to the Gamble!");

    loop {
        let mut is_win = true;
        println!("Please choose your your_weapon!");
        println!("1. Paper");
        println!("2. Rock");
        println!("3. Scissors");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read new input!");
        println!("You choose: {}", choice);
        let your_choice: u32 = choice.trim().parse().unwrap();
        let mut rng = thread_rng();
        let secret: u32 = rng.gen_range(1..4);
        println!("System: {}", secret);
        let your_weapon = Weapon::matching(&your_choice);
        let system_weapon = Weapon::matching(&secret);
        println!("Yours: {}", your_weapon.value());
        println!("System: {}", system_weapon.value());
        if your_weapon.value() == system_weapon.value() {
            println!("Tie!");
            is_win = false;
        } else {
            if your_weapon.value() == Weapon::Rock.value() {
                if system_weapon.value() == Weapon::Paper.value() {
                    println!("Sorry, this time not for you!");
                    is_win = false;
                } else {
                    println!("Chicken Winner!!");
                }
            } else if your_weapon.value() == Weapon::Paper.value() {
                if system_weapon.value() == Weapon::Scissors.value() {
                    println!("Sorry, this time not for you!");
                    is_win = false;
                } else {
                    println!("Chicken Winner!!");
                }
            } else {
                if system_weapon.value() == Weapon::Rock.value() {
                    println!("Sorry, this time not for you!");
                    is_win = false;
                } else {
                    println!("Chicken Winner!!");
                }
            }
        }
        if is_win {
            break;
        }
    }
}
