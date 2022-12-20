use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let min = 1;
    let max = 100;

    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("I have a number between in my mind!");
    println!("It's bigger than {} and smaller than {}.", min-1, max+1);
    println!("Try to guess it! I promiss I won't cheat.");
    println!();

    println!("My number: {secret_number}");
    println!("Do you have your guess? Input it now:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    let guess: u32 = guess.trim().parse().expect("Number, please!");

    println!();
    println!("Your guess: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Equal => println!("Good job!"),
        Ordering::Greater => println!("Too high!"),
    }
    println!("My number was {secret_number}.");
}
