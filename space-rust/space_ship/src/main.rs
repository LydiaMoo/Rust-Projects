extern crate dialoguer;

use dialoguer::{Input};
use std::process;

struct Room {
    name: String,
    view: String,
    roomNumber: i32,
    active: bool,
}

fn main() {

    let viewing_platform = Room {
        name: "Viewing Platform".to_string(),
        view: "Stars forever".to_string(),
        roomNumber: 2,
        active: false
    };

    let air_lock = Room {
        name: "Air Lock".to_string(),
        view: "Go outside and see".to_string(),
        roomNumber: 3,
        active: false
    };

      let canteen = Room {
        name: "Canteen".to_string(),
        view: "Too Busy focus on food".to_string(),
        roomNumber: 4,
        active: false
    };

      let flight_deck = Room {
        name: "Flight deck".to_string(),
        view: "Where we're going".to_string(),
        roomNumber: 5,
        active: false
    };

      let entrance = Room {
        name: "Entrance".to_string(),
        view: "You just got here, look around".to_string(),
        roomNumber: 3,
        active: false
    };

      let engine_room = Room {
        name: "Engine Room".to_string(),
        view: "No windows here just crazy machines".to_string(),
        roomNumber: 3,
        active: false
    };

      let sleeping_quarters = Room {
        name: "Sleeping quarters".to_string(),
        view: "Cosy beds".to_string(),
        roomNumber: 3,
        active: false
    };

    let ship = [viewing_platform, air_lock, canteen, flight_deck, entrance, engine_room, sleeping_quarters];

    println!("Hello, welcome aboard. Firstly please could you tell us your name?");
    let name = Input::new("Your name").interact().unwrap();
    println!("Hello {}, let's prepare for an exciting adventure, would you like to start? (Please type y or n)", name);
    let response = Input::new("Your response").interact().unwrap();

    if response == "y" {
        println!("Awesome, let's get going.");
    } else {
        println!("That's a shame.");
        process::exit(0x0100);
    }

    let health = 1; 

    let mut location = 1; 

    while health == 1 {

        println!("{} you are in the {}. In any room you can move to the outside or inside of the ship and clockwise or anti.", name, ship[location].name);
        let response = Input::new("Your response").interact().unwrap();

        location = if response == "clockwise" {
            if location == 6 { 
                println!("resetting, {}", location);
                0
            } else {
                println!("incrementing, {}", location);
                location + 1
            }
        } else if response =="anti" {
            if location == 0 {
                println!("resetting, {}", location);
                6
            } else {
                println!("decrementing, {}", location);
                location - 1
            }
        } else {
            process::exit(0x0100);
        };
        println!("location number, {}", location); 
    };
    //println!("{} you are now in the {}", name, ship[location]);
    
}
