// Module Tree

// NOTE: lib.rs
mod front_of_house {

    // NOTE: can be in src/front_of_house.rs
    pub mod hosting { // Making module hosting public
        // NOTE: can be in src/front_of_house/hosting.rs
        pub fn add_to_waitlist() {} // making function public
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // uses relative path from parent module
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }            
        }
    }

    // NOTE: All enum variants are public if the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use can import structs directly
use std::collections::HashMap;
use std::collections::HashSet as Set; // give imported struct a name

use crate::front_of_house::hosting;

// NOTE: eat_at_resturant() is defined in the same module as front_of_house
pub fn eat_at_resturant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Absolute path simplified with use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // attribute is private, code won't work

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1,2);
    let mut set = Set::new();
    set.insert(1);
}


// Instead of this
//use std::io;
//use std::io::Write;

// We can use this
use std::io::{self, Write};

// And we can also import all public items too
use std::collections::*;
