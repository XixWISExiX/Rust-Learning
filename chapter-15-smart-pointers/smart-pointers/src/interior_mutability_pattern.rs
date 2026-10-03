
pub fn run() {
    println!("interior_mutability_pattern.rs");
    example1();
    example2();
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgant warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn example1() {
    let x = 5;
    //let y = &mut x; // This is not allowed by the compiler
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    // NOTE: Having a reference counter of a reference cell
    // allows for multiple pointers to point to a single mutating value.
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn example2() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // NOTE: a, b, and c share the same components, so this is space efficient.
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}



#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>, // Hidden mutability is now allowed
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]), // Hidden mutability is now allowed
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            //let mut one_borrow = self.sent_messages.borrow_mut();
            //// NOTE: This resuls in an error because we are
            //// borrowing the mutation variable again.
            //let mut two_borrow = self.sent_messages.borrow_mut();
            //one_borrow.push(String::from(message));
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // adds string to message vector under the hood (mutation)
        // NOTE: This code would error out if RefCell wasn't used.
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
