use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome To Our Rust Guessing Game");
    println!("Your Task  => Guess the Number");
    println!("Enter your Guess = -_-");
    
    //generating Secret Number
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The Secret Number is :{}", secret_number);



    //Creating Guess
    let  guess:u32 = guess.trim().parse().expect("Please type a Number");
    io::stdin().read_line(&mut guess).expect("Failed To Read Line");
    println!("You Guessed :{}", guess);


    //Matching Guess in the Game
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win!"),
    }

}
