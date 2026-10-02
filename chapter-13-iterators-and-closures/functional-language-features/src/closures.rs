use std::thread;

pub fn run() {
    println!("closures.rs");
    run_shirts_code();
    example1();
    example2();
    example3();
    example4();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,    
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // NOTE: Calling a closure on an Option.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn run_shirts_code() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    
}


fn example1() {
    // Optional data types are present in closer
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        num
    };

    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // Closure doesn't need type, but must be used in
    // order for the compiler to declare types.
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let a: u32 = 1;
    let result = add_one_v3(a);
    println!("a + 1 = {result}");
    let b: i64 = 1;
    //add_one_v3(b); // would result in error due to type mismatch
    let result = add_one_v4(b);
    println!("b + 1 = {result}");

    let add_b_and_one = |x| b + x + 1;
    let c: i64 = 2;
    let result = add_b_and_one(c);
    println!("c + 1 + b (from outside function) = {result}");
}


fn example2() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // You can put macros in closure too.
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    only_borrows();


    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    borrows_mutably();
    // Borrowed Mutability ends after the println!
    println!("After calling closure: {list:?}");

}


fn example3() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // The move operator moves all variables used in the 
    // closure into the thread
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn example4() {
    let mut list = [
        Rectangle {width: 10, height: 1 },
        Rectangle {width: 3, height: 5 },
        Rectangle {width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let string = String::from("s");
    let mut num_sort_operations = 0;

    // sorts key by the width and uses
    // a FnMut
    list.sort_by_key(|r| r.width);
    list.sort_by_key(|r| {
        // cannot do a move operation into the function though.
        //sort_operations.push(string);    

        num_sort_operations += 1;
        sort_operations.push(2);
        r.width
    });
    println!("list of rectangles: {list:#?}");
    println!("sort operations array: {sort_operations:#?}");
    println!("num of sorted operations: {num_sort_operations:#?}");
}
