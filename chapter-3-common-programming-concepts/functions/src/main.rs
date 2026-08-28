fn main() {
    println!("Hello, world!");

    another_function();
    param_function(10, 'e');
    statement_function();
    expression_function();

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);

    expressions();
    loops();
}

fn another_function() {
    println!("Another function")
}

// A function can have parameters (arguments) that can be
// used inside the given function.
fn param_function(x: i32, character: char) {
    println!("The value of x is: {}", x);
    println!("The value of character is: {}", character);
}

fn statement_function() {
    let x = 6;
    println!("Statement = {}", x);
}

fn expression_function() {
    let x = {
        let y = 1;
        y + 1 // No return here, it's assumed with now semi-colon
    };
    println!("Expression = {}", x);
}

fn five() -> i32 {
    5 // Another example of an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn expressions() {
    let number = 3;

    // Basic if/else conditional
    if number < 5 {
        println!("True conditional");
    } else {
        println!("False conditional");
    }

    // Not equal to conditional
    if number != 0 {
        println!("Number is not 0");
    } 

    // Using else if statements
    if number % 4 == 0 {
        println!("Number divisible by 4");
    } else if number % 3 == 0 {
        println!("Number divisible by 3");
    } else if number % 2 == 0 {
        println!("Number divisible by 2");
    } else {
        println!("Number isn't divisible by 4, 3, or 2");
    }

    // Ternary
    let condition = true;
    let number = if condition {6} else {7}; // note both values in ternary must be same type
    // let number = if condition {6} else {"seven"}; // this errors out
    println!("The value of the number is {}", number);
}

fn loops() {
    let mut i = 0;
    // Loop runs forever until explicitly told to stop
    loop {
        if i == 10 { break } // can have one liner conditionals
        println!("again!");
        i+=1;
    }

    // You can also return values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 // this loop returns this content
        }
    };
    println!("Result of value from loop: {}", result);

    // You can also structure the code to get out of the outer most loop
    // with labeled loops.
    let mut count = 0;
    'outer: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        'inner: loop {
            println!("remaining = {remaining}");
            if remaining == 9 { break 'inner; }

            if count == 2 { break 'outer; }

            remaining -= 1;
        }
        count+=1;
    }
    println!("End count = {}", count);

    // There are also while loops
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("Out of while loop!");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // There are for each loops
    for element in arr {
        println!("Array element: {}", element);
    }

    // There are for loops (last number is exclusive like always)
    for num in (1..4) {
        println!("{}!", num);
    }
    println!();
    // Can go in reverse too
    let x = 2;
    let y = 5;
    for num in (x..y).rev() {
        println!("{}!", num);
    }
}
