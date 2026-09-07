use std::net::IpAddr;

pub fn run() {
    println!("when_to_panic.rs");

    // Case where you have a result that will always work.
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
    let guess: Guess = Guess::new(4); // valid
    println!("We have a valid guess of {}!", guess.value());
    //let guess: Guess = Guess::new(200); // error
}

pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value outside guessing range of 1 and 100, got {value}");
        }
        Guess {value}
    }

    fn value(&self) -> i32 {
        self.value
    }
}
