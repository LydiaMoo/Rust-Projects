extern crate dialoguer;

use dialoguer::{Input};
use std::process;

use std::time::{Duration};
use std::thread::sleep;

struct Room {
    name: String,
    view: String,
    action: String,
    desc: String,
    //roomNumber: i32,
    //active: bool,
}

fn airlock() {
    //let _now = Instant::now();
    sleep(Duration::new(2, 0));
    println!("\nOut here you can get a great view of the space-ship and the earth looks beautiful.");
    sleep(Duration::new(4, 0));
    println!("\nFloating out here is calm and relaxing.");
    sleep(Duration::new(6, 0));
    println!("\nWhoops, air is running out, time to go back inside now, before it's too late.");
}

fn main() {

    let viewing_platform = Room {
        name: "Viewing Platform".to_string(),
        view: "\nStars forever".to_string(),
        action: "watch".to_string(),
        desc: "\nIs that Father Christmas going over Australia".to_string()
        //roomNumber: 2,
        //active: false
    };

    let air_lock = Room {
        name: "Air Lock".to_string(),
        view: "\nGo 'outside' and see".to_string(),
        action: "outside".to_string(),
        desc: "\nYou're now entering the airlock and preparing to go outside, putting on your space-suit and heading outside.".to_string()
        //roomNumber: 3,
        //active: false
    };

      let canteen = Room {
        name: "Canteen".to_string(),
        view: "\nToo Busy focus on food".to_string(),
        action: "watch".to_string(),
        desc: "\nIs that Father Christmas going over Australia".to_string()
        //roomNumber: 4,
        //active: false
    };

      let flight_deck = Room {
        name: "Flight deck".to_string(),
        view: "\nWhere we're going".to_string(),
        action: "watch".to_string(),
        desc: "\nIs that Father Christmas going over Australia".to_string()
        //roomNumber: 5,
        //active: false
    };

      let entrance = Room {
        name: "Entrance".to_string(),
        view: "\nYou just got here, look around".to_string(),
        action: "watch".to_string(),
        desc: "\nIs that Father Christmas going over Australia".to_string()
        //roomNumber: 3,
        //active: false
    };

      let engine_room = Room {
        name: "Engine Room".to_string(),
        view: "\nNo windows here just crazy machines but, what's that dial, something's wrong. \n Hit 'restart' to see if we can fix it.".to_string(),
        action: "restart".to_string(),
        desc: "\n Thump, wallap, bang. Phew, close one there, looks like everything's going to be okay".to_string()
        //roomNumber: 3,
        //active: false
    };

      let sleeping_quarters = Room {
        name: "Sleeping quarters".to_string(),
        view: "\nCosy beds".to_string(),
        action: "watch".to_string(),
        desc: "\nIs that Father Christmas going over Australia".to_string()
        //roomNumber: 3,
        //active: false
    };

    let ship = [viewing_platform, air_lock, canteen, flight_deck, entrance, engine_room, sleeping_quarters];

    println!("Hello, welcome aboard. Firstly please could you tell us your name?");
    let name = Input::new("Your name").interact().unwrap();
    println!("Hello {}, let's prepare for an exciting adventure, would you like to start? (Please type y or n)", name);
    let response = Input::new("Your response").interact().unwrap();

    if (response.to_lowercase()) == "y" {
        println!("Awesome, let's get going.");
        println!("\nIn any room you can move around the space-ship 'left', or 'right', look at the 'view' and action whatever is in quote marks");
    } else {
        println!("\nThat's a shame.\n");
        process::exit(0x0100);
    }

    let health = 1; 

    let mut location = 1; 

    println!("\n{} you are in the {}.", name, ship[location].name);

    while health == 1 {

        
        let response = Input::new("\nYour response").interact().unwrap();

        location = if (response.to_lowercase()) == "right" {
            if location == 6 { 
                println!("\n{} you are in the {}.", name, ship[0].name);
                0
            } else {
                println!("\n{} you are in the {}.", name, ship[location+1].name);
                location + 1
            }
        } else if response == "left" {
            if location == 0 {
                println!("\n{} you are in the {}.", name, ship[6].name);
                6
            } else {
                println!("\n{} you are in the {}.", name, ship[location-1].name);
                location - 1
            }

        } else if (response.to_lowercase()) == ship[location].action 
            {
                if (response.to_lowercase()) == "outside" {
                    println!("\n{}", ship[location].desc);
                    airlock();
                    location
                } else {
                    println!("\n{}", ship[location].desc);
                    location
            }
        } else if (response.to_lowercase()) == "view" {
            println!("\n{}", ship[location].view);
            location
        } else {
            process::exit(0x0100);
        };
        //println!("location number, {}", location); 
    };
    //println!("{} you are now in the {}", name, ship[location]);
    
}
