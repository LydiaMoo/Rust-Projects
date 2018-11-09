extern crate dialoguer;

use dialoguer::{Input};

fn main() {
    println!("Hello, well aboard. Firstly please could you tell us your name?");
    let name = Input::new("Your name").interact().unwrap();
    println!("Hello {}, let's prepare for an exciting adventure, would you like to start? (Please type y or n)", name);
    let response = Input::new("Your response").interact().unwrap();

    if response == "y" {
        println!("Awesome, let's get going.");
    } else {
        println!("That's a shame.");
    }
}
