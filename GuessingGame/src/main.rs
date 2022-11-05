use std::io;


fn main() {
    println!("Welcome To Our Rust Guessing Game");
    println!("Your Task  => Guess the Number");
    println!("Enter your Guess = -_-");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed To Read Line");

    println!("You Guessed :{}", guess);

}
