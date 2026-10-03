
pub fn run() {
    println!("pointer_reference_counter.rs");
    example1();
}

use std::rc::Rc;

enum List {
    //Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn example1() {
    // NOTE: Doesn't work because Box cannot share ownership
    //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // clone that increases the reference count to a
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // When a goes out of scope the reference count becomes 0 and the pointer
    // gets deallocated.
}
