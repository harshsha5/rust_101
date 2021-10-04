use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1,10);

    loop {

        println!("Guess the number?");
        
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("Please type a number");    
        
        println!("You guessed: {}",guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Your number was too small".red()),
            Ordering::Equal => {
                println!("{}","Your number was an exact match".green());
                break;
            },
            Ordering::Greater => println!("{}","Your number was too big".red()),
        }
    }
    println!("The secret number was: {}",secret_number); 
}
