use std::io;
use rand::Rng;

fn main() {
    let min = 1;
    let max = 100;

    let secret_number = rand::thread_rng().gen_range(min..=max);

    println!("I have a number between in my mind!");
    println!("It's bigger than {} and smaller than {}.", min-1, max+1);
    println!("Try to guess it! I promiss I won't cheat.");
    println!();

    println!("Do you have your guess? Input it now:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!();
    print!("Your guess: {guess}");
    println!("My number is {secret_number}");
}
