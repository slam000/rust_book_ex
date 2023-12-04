use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Create a new, empty string
    let mut guess = String::new();

    // Read the next line from standard input (stdin) and place it into the string we created
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
