extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, Guess the number which is between 1 and 101!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

   // println!("The secret number is {}", secret_number);
   
    loop {
        println!("Please input your guess.");
        //mut allows it to be mutable
        let mut guess = String::new(); 
        //std::io::stdin()
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess); 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
