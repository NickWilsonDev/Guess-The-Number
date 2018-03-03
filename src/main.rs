use std::io;
// bring in io library from standard library


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let declares variable, mut allows variable to be mutable
    let mut guess = String::new();

    // & means reference, references are immutable by default
    io::stdin().read_line(&mut guess).expect("Failed to read line");


    // how to sub data into strings
    println!("You guessed: {}", guess);

}
