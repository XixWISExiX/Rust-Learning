pub fn run() {
    println!("write-tests.rs");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        //self.width < other.width && self.height < other.height // NOTE: buggy fn case
    }
}

fn greeting(name: &str) -> String {
    format!("Hello {name}!")
    //format!("Hello!") // NOTE: buggy fn case
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.")
        }
        Guess { value }
    }
    fn new2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.")
        }
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.")
        }
        Guess { value }
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 7,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_smaller_assert_eq() {
         let larger = Rectangle {
            width: 7,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_eq!(larger.can_hold(&smaller), true);
    }

    #[test]
    fn smaller_cannot_hold_larger_assert_ne() {
        let larger = Rectangle {
            width: 7,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_ne!(smaller.can_hold(&larger), true);
        //assert_eq!(!smaller.can_hold(&larger), true);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic] // the test should result in a panic
    fn greater_than_100() {
        Guess::new(200);
        //Guess::new(20); // NOTE: doesn't throw error, results in failed test
    }

    // Check if we get a particular error from the tested fn
    #[test]
    // If panic contains this string, then the test passed
    #[should_panic = "less than or equal to 100"] // the test should result in a panic
    fn greater_than_100_pt2() {
        Guess::new2(200);
        //Guess::new2(0); // NOTE: throws incorrect error, results in failed test
    }

    // Can throw helper debug text upon failing test
    #[test]
    fn it_works() ->  Result<(), String> {
        let result = add(2, 2);
        //let result = add(2, 3); // for the test to fail
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
 
}
