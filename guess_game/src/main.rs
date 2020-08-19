use std::io;
use rand::Rng;


// Rust Types are following
fn guessNumber() -> i32 {
    let secret = rand::thread_rng().gen_range(1, 101);
    return secret;
}


fn main() {
    
    println!("Guess Number");
    println!("Please input a number");
    let mut guess = String::new();
    
    // This is without the use of USE since std::io is an inbuilt crate
    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    
    // This is with the use of USE since std::io is an inbuilt crate   
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let secret = guessNumber();
    // Create a secret word from this number to this
    println!("Your guess: {}", guess);
    println!("Your secret number generated: {} ", secret);
    
}
