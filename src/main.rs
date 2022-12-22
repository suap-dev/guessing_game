use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let min = 1;
    let max = 100;

    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("I have a number between in my mind!");
    println!("It's bigger than {} and smaller than {}.", min - 1, max + 1);
    println!("Try to guess it! I promiss I won't cheat.");
    println!();

    // println!("My number: {secret_number}");

    println!("Your guess:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        // let guess: u32 = guess.trim().parse().expect("Number, please!");
        let guess: u32 =
            match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Hmm, this is what we need.");
                    println!("We're looking for an integer in numeric form.");
                    println!("Try again. :)");
                    continue
                }
            };

        println!();
        print!("Your guess of {guess} is too ");
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("low"),
            Ordering::Equal => {
                println!("good to be true!");
                break;
            }
            Ordering::Greater => print!("high"),
        }

        println!(", try again: ");
    }
    println!("My number was {secret_number}!");
}
