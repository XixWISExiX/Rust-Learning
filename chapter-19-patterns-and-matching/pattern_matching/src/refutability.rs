pub fn run() {
    println!("refutability");
    refutable();
    irrefutable();
}

fn refutable() {
    let some_option_value: Option<i32> = Some(32);
    //let Some(x) = some_option_value; // Errors Out

    // NOTE: Refutable patterns must use let else (this removes the Option type)
    let Some(x) = some_option_value else {
        return;
    };
    println!("refutable x is {}", x);
}

fn irrefutable() {
    // NOTE: Results in a warning
    //let x = 5 else {
    //    return;
    //};

    // NOTE: for Irrefutable patterns, normal syntax works obviously
    let x = 5;
    println!("irrefutable x is {}", x);
}
