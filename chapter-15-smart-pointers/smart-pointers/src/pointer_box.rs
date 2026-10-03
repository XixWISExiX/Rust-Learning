
pub fn run() {
    println!("pointer_box.rs");
    example1();
    example2();
    example3();
    example4();
}

// NOTE: Example of the lists type.
#[derive(Debug)]
enum List {

    // Rust cannot calculate the amount of space needed here
    // There fore this code errors out.
    //Cons(i32, List),

    // Space needed here is calculatable because we can also
    // include pointer size.
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn example1() {
    let b = Box::new(5);
    // b points to the integer 5 on the heap.
    println!("b = {b}");

    // NOTE: This is the cons list data structure in action
    // that comes from Lisp programming dialects.
    // You can think of it like a Linked List, but recursive.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Cons list: {list:?}");
}

// NOTE: How Box looks like under the hood
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

// NOTE: this overrides the * operator
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn example2() {
    let x = 5;
    let y = &x; // NOTE: y is a pointer to x
    let z = Box::new(x); // NOTE: z is a pointer to a copied value of x, whoes on the heap
    let w = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn example3() {
    let m = MyBox::new(String::from("Rust"));
    // passes a reference to MyBox and dereference behind
    // the scenes turns &MyBox to &String and then to a &str
    // NOTE: This is called Deref Coercion
    hello(&m);
    // hello(&(*m)[..]); // NOTE: doing this behind the scenes
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // NOTE: This function cannnot be called manually.
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn example4() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("additional stuff"),
    };
    println!("CustomSmartPointer created");

    // NOTE: We can call std::mem::drop, however this is different
    // than the drop trait.
    drop(c);

    // dropped pointers should now print stuff.
    // variables are dropped in reverse order
}
