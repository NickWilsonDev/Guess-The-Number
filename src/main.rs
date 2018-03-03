extern crate rand;

use std::io;
use rand::Rng;
// bring in io library from standard library


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let declares variable, mut allows variable to be mutable
    let mut guess = String::new();

    // & means reference, references are immutable by default
    io::stdin().read_line(&mut guess).expect("Failed to read line");


    // how to sub data into strings
    println!("You guessed: {}", guess);

}
