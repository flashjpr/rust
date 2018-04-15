extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let answer = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Insert a number: ");

        //  take user input 
        // let - create vars
        //  :: new - associated method of String
        // -> created a mutable var guess with a value of empty string
        let mut guess = String::new(); 

        io::stdin().read_line(&mut guess)
        .expect("Error reading the line");

        // guess is a string, answer is a number, so shadow guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small !"),  // re-do the game 
            Ordering::Greater => println!("Too big !"), // re-do the game
            Ordering::Equal => {
                println!("Correct !");
                break;
            }
        }
        
    }
    
}

// // define a function that reads user input
// fn get_user_input(&input) {
//     io::stdin().read_line(&mut input)
//        .expect("Error reading the line");
// } 