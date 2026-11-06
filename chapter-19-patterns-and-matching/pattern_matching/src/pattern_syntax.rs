pub fn run() {
    println!("pattern_syntax");
    matching_literals();
    matching_named_vars();
    matching_multiple_patterns();
    matching_range_of_vars();
    destructure_structs();
    destructure_enums();
    destructing_nested_struct();
    other_underscore_usecases();
    remaining_parts();
    match_with_conditionals();
    at_bindings();
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_vars() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

fn matching_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_range_of_vars() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructure_structs() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("single val a {a}");
    assert_eq!(0, a);
    println!("single val b {b}");
    assert_eq!(7, b);

    // NOTE: Has to be the actual var names from inside Point for this syntax to work
    let Point { x, y } = p;
    println!("single val x {x}");
    assert_eq!(0, x);
    println!("single val y {y}");
    assert_eq!(7, y);

    
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } =>  println!("On the x axis at {x}"), // when y is 0
        Point { x: 0, y } => println!("On the y axis at {y}"), // when x is 0
        Point { x, y } => { // else
            println!("On neither axis: ({x}, {y})");            
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructure_enums() {
    let msg = Message::Quit;
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

fn destructing_nested_struct() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10});
    println!("Split struct and enum:");
    println!("feet {feet}");
    println!("inches {inches}");
    println!("x {x}");
    println!("y {y}");
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn other_underscore_usecases() {
    // NOTE: _ with function
    foo(3, 4);

    // NOTE: _ with any value
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");

    // NOTE: _ with args in match
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // NOTE: leading _ on variable name
    let _x = 5; // we don't get a warning on not using x
    let y = 10;

    let s = Some(String::from("Hello!"));

    // NOTE: This takes ownership
    //if let Some(_s) = s {
    //    println!("found a string");
    //}
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

fn remaining_parts() {
    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is {x}"),
    }
    
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    match numbers {
        //(.., second, ..) => { // NOTE: Errors out
        (_, second, ..) => {
            println!("Some numbers: {second}")
        },
    }
}



fn match_with_conditionals() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        //(4 | 5 | 6) if y => println!("yes"), // NOTE: what the below code is under the hood
        4 | 5 | 6 if y => println!("yes"),        
        _ => println!("no"),
    }
}

enum HelloMessage {
    Hello { id: i32 },
}

fn at_bindings() {
    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello { id: id @ 3..=7 } => {
             println!("Found an id in range: {id}")
        }
        HelloMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        HelloMessage::Hello { id } => println!("Found some other id: {id}"),
    }
}
