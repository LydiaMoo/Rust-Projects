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
    println!("\nFloating out here is so calm and relaxing.");
    sleep(Duration::new(6, 0));
    println!("\nWhoops, air is running out...");
    sleep(Duration::new(6, 0));
    println!("\nTime to go back inside now! before it's too late.");
}

fn main() {

    let viewing_platform = Room {
        name: "Viewing Platform".to_string(),
        view: "\nOh my God! Look at that picture over there! There's the Earth coming up. Wow, that's pretty. Learn 'more'.".to_string(),
        action: "more".to_string(),
        desc: "\nYou're not the first to see an Earthrise. Earthrise was a photograph taken by the astronaut Bill Anders and shows the Earth rising up from behind the grey surface of the moon in front of an impenetrable black background. It is one of a number of photographs taken on board the United States spacecraft Apollo 8 as it orbited the moon on 24 December 1968".to_string()
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
        view: "\nOn the counter there are some shiny foil packets. Maybe we could 'open' one and have a look to see what the crew eats at Christmas on the ISS".to_string(),
        action: "open".to_string(),
        desc: "\nYou pick up one of the packets, on the front it says FULL TURKEY DINNER. You rip open the top of the packet and the smell of the most wonderful roast dinner with all the trimmings overwhelms you. Yummy.".to_string()
        //roomNumber: 4,
        //active: false
    };

      let flight_deck = Room {
        name: "Flight deck".to_string(),
        view: "\nOoooh, look at those controls.\nRows and rows of dials and switches and...\n\nthere's a big red 'button' too.".to_string(),
        action: "button".to_string(),
        desc: "         .--._.--.--.__.--.--.__.--.--.__.--.--._.--.
       _(_      _Y_      _Y_      _Y_      _Y_      _)_
      [___]    [___]    [___]    [___]    [___]    [___]
      /:' |    /:' |    /:'  |   /:'  |   /:' |    /:' |  
     |::. |   |::.  |  |::.  |  |::.  |  |::.  |  |::.  |
      |::./    |::./    |::./    |::./    |::./    |::./
       '='      '='      '='      '='      '='      '='

-------------------------------------------------------------
\n You are lucky that was just the Christmas lights!".to_string()
        //roomNumber: 5,
        //active: false
    };

      let entrance = Room {
        name: "Entrance".to_string(),
        view: "\nYou just got here, look around the ISS some more or you could 'leave'.".to_string(),
        action: "leave".to_string(),
        desc: "\nGood bye".to_string()
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
        view: "\nYou can see some lovely cosy beds. \nSShhh, somebody's sleeping over there. \n \nWait red pyjamas with a fluffy collar...should we look 'closer'?".to_string(),
        action: "closer".to_string(),
        desc: "\nOooooh, false alarm, it's not Father Christmas, it's just Tim Peake playing a joke on us.".to_string()
        //roomNumber: 3,
        //active: false
    };

    let ship = [viewing_platform, air_lock, canteen, flight_deck, entrance, engine_room, sleeping_quarters];

    println!("\nHello, welcome aboard it's Christmas on the ISS 2027 and you've got some exploring to do.
         .--._.--.--.__.--.--.__.--.--.__.--.--._.--.
       _(_      _Y_      _Y_      _Y_      _Y_      _)_
      [___]    [___]    [___]    [___]    [___]    [___]
      /:' |    /:' |    /:'  |   /:'  |   /:' |    /:' |  
     |::. |   |::.  |  |::.  |  |::.  |  |::.  |  |::.  |
      |::./    |::./    |::./    |::./    |::./    |::./
       '='      '='      '='      '='      '='      '='

-------------------------------------------------------------
\nFirstly please could you tell us your name?"
);
    let name = Input::new("\nYour name").interact().unwrap();
    println!("\n{}, let's prepare for an exciting Christmas adventure in space. \nIn any room you can move around the space-ship 'left', or 'right', look 'around' and action whatever is in quote marks. \nWould you like to start? (Please type y or n)", name);
    let response = Input::new("\nYour response").interact().unwrap();
    if (response.to_lowercase()) == "y" {
        println!("\nAwesome, let's get going.");
    } else {
        println!("\nThat's a shame.\nGoodbye.");
        process::exit(0x0100);
    }

    let health = 1; 

    let mut location = 1; 

    println!("\n\n{} you are in the {}.", name, ship[location].name);

    while health == 1 {

        let response = Input::new("\nYour response").interact().unwrap();

        location = match response.to_lowercase().as_ref() {
            "right" => if location == 6 { 
                println!("\n{} you are in the {}.", name, ship[0].name);
                0
            } else {
                println!("\n{} you are in the {}.", name, ship[location+1].name);
                location + 1
            },

            "left" => if location == 0 {
                println!("\n{} you are in the {}.", name, ship[6].name);
                6
            } else {
                println!("\n{} you are in the {}.", name, ship[location-1].name);
                location - 1
            },

            bibble if bibble == ship[location].action => 
                if response.to_lowercase() == "outside" {
                    println!("\n{}", ship[location].desc);
                    airlock();
                    location
                } else {
                    println!("\n{}", ship[location].desc);
                    location
            },

            "around" => {println!("\n{}", ship[location].view);
                location},

            "where" => {println!("\n{}", ship[location].name);
                location},

            "leave" => process::exit(0x0100),

            _ => {println!("Do some more things");
                location}
        };
    };  
}
