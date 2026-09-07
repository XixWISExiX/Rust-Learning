fn main() {
    defining_an_enum();
    println!();
    the_match_control_flow_construct();
    println!();
    consise_control_flow_with_if_let();
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
struct Other {
    data1: u8,
    data2: u8,
}

// Enums can embed data types including structs
#[derive(Debug)]
enum IpAddrImproved {
    V4(u8, u8, u8, u8),
    V6(String, Other), // Example of struct embedding
}

enum Message {
    Quit, // has no data
    Move { x: i32, y: i32 }, // has fields like struct
    Write(String), // single string
    ChangeColor(i32, i32, i32), // 3 values
}

// NOTE the above does the same thing as below
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// We can also implement enums
impl Message {
    fn hello(&self) {
        println!("Hello!");
    }
}

// Standard lib implementation of optional enum
enum CustomOption<T> {
    None,
    Some(T),
}

fn defining_an_enum() {
    let four = IpAddrKind::V4;        
    let six = IpAddrKind::V6;        

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let other = Other {
        data1: 1,
        data2: 2,
    };

    let home = IpAddrImproved::V4(127, 0, 0, 1);
    let loopback = IpAddrImproved::V6(String::from("::1"), other);

    dbg!(loopback);

    let m = Message::Write(String::from("hello"));
    m.hello();

    // Using optional enums
    let some_number = Some(5);
    let some_char = Some('c');
    let absent_number: Option<i32> = None;

    // However you cannot add optional and non optional types directly
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = Some(5);
    //let sum = x + y; // Errors out
    //let sum = y + z; // This also errors out
    let sum = x + y.unwrap(); // unwrap casts back to type T
    println!("Optional addition unwrap works! {}", sum);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Texas,
    Oklahoma,
    California,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Other,
}

fn the_match_control_flow_construct() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);    
    println!("Value of coin is: {}", value);

    let coin = Coin::Quarter(UsState::Texas);
    let value = value_in_cents(coin);    
    println!("Value of coin is: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six is: {}", six.unwrap());


    // Match statements must handle all cases.
    run_dice_game(9);
    run_dice_game(3);
    run_dice_game(7);
}

fn value_in_cents(coin: Coin) -> u8 {
    // match statements are very similar to switch case statements
    match coin {
        Coin::Penny => {
            println!("THE PENNIES ARE GONE!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
        _ => 0, // default value
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn run_dice_game(roll: i32) {
    match roll {
        3 => println!("You got a hat!"),
        7 => println!("You got a cat!"),
        _ => (), // In this case, which is else, do nothing
    }
}

// If let statements are just simpler match statements, that's it.
fn consise_control_flow_with_if_let() {
    // Say you want to do the following
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),       
        _ => (),
    }
    
    // if let syntax allows you to combine let and match statments
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Texas);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count+=1,
    }

    // or
    let coin = Coin::Quarter(UsState::Texas);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count+=1;
    }

    let coin = Coin::Quarter(UsState::Texas);
    println!("{}", describe_state_quarter(coin).unwrap());
    let coin = Coin::Quarter(UsState::Oklahoma);
    println!("{}", describe_state_quarter(coin).unwrap());
    let coin = Coin::Penny;
    describe_state_quarter(coin);

}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Texas => year >= 1845,
            UsState::Oklahoma => year >= 1907,
            UsState::California => year >= 1850,
            UsState::NewYork => year >= 1788,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // Assigning a variable from an if let statement
    //let state = if let Coin::Quarter(state) = coin {
    //    state
    //} else {
    //    return None;
    //};

    // Same as above commented out code
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is is relatively new."))
    }
}
