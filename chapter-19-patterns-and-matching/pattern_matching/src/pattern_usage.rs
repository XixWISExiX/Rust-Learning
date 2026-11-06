pub fn run() {
    println!("pattern_usage");
    basic_matches();
    println!("=====");
    loops();
    println!("=====");
    function_as_pattern();
}

fn basic_matches() {
    let x: Option<i32> = Some(67);
    let x = match x {
        None => None, // no value
        Some(i) if i == 67 => Some(i + 1), // some value of i32 is 67
        _ => Some(21), // other cases
    };
    
    println!("x after match: {}", x.unwrap());



// let PATTERN = EXPRESSION;
    let x = 5;
    // NOTE: 3 var tuple
    let (x, y, z) = (1, 2, 3);



    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // NOTE: Mixing if let, else if, else if let, and else
    // compiler doesn't check for exhaustiveness
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

}

fn loops() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    println!("while let done");

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

// NOTE: checks for tuple pattern undert the hood
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn function_as_pattern() {
    let point = (3, 5);
    print_coordinates(&point);
}


