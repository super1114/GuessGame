use std::io;
//use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is {}", secret_number);
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess: {}", guess);
}