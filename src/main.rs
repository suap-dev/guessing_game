use std::io;

fn main() {
    println!("I have a number in mind, try to guess it! I promiss I won't cheat.");
    println!("Do you have your guess? Input it now:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("Your guess: {guess}");
}
