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
        let guess: u32 = guess.trim().parse().expect("Number, please!");

        println!();
        println!("Your guess: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too low!"),
            Ordering::Equal => {
                println!("Good job!");
                break;
            }
            Ordering::Greater => print!("Too high!"),
        }

        println!(" Try again: ");
    }
    println!("My number was {secret_number}.");
}
