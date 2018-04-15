extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let answer = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Take a guess: ");
        let mut guess = String::new(); 

        io::stdin().read_line(&mut guess)
        .expect("Error reading the line");

        // guess is a string, answer is a number, so shadow guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => continue, // _ -> catchall
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small !"),  
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("Correct !");
                break;
            }
        }
        
    }
    
}