fn main() {
    variable_scope();
    references_and_borrowing();
    slice_type();
}

fn variable_scope() {
    let x = 6; // variable assigned on the variable_scope level of the stack.
    {
        let y = 7; // variable assigned to this lower scope level

        let stack_string = "hello"; // can never be a mutable variable
        let mut heap_string = String::from("hello"); // can be a mutable variable
        heap_string.push_str(", world!");
        println!("{}", heap_string);
    } // heap_string variable is out of scope and automatically deleted.

    {
        let a = 5; // a is 5
        let b = a; // b is 5

        let s1 = String::from("hello"); // s1 is pointing to "hello"
        let s2 = s1; // s1 pointer ownership moved to s2, shallow copy
        //println!({}, s1); // will error out because pointer was moved.

        let s3 = String::from("hello"); // s3 is pointing to "hello"
        let s4 = s3.clone(); // s4 pointing to another "hello" than s3, deep copy
        println!("s3 = {}, s4 = {}", s3, s4);
        // NOTE: Deep copies only work on heap allocated objects
    }
    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s); // errors out
    let a = 5;
    makes_copy(a);

    let s1 = gives_ownership(); // ownership value
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // ownership basically given to s3 from s2

    let (s1, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s1, len);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    let s = String::from("give");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn references_and_borrowing() {
    let s1 = String::from("ref");
    // In the below case, s points to s1 which points the the string on the heap.
    let len = calculate_only_length(&s1); // this function takes s1 without taking ownership
    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("ref");
    change_string(&mut s2); // this function takes s1 without taking ownership
    println!("The new s2 = {}", s2);

    // We can have mutiple immutable references
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 = {}, r2 = {}", r1, r2);

    // However we can only have one mutable value per scope.
    let r3 = &mut s2; 
    //let r2 = &mut s2; // this code errors out
    // NOTE: r1 and r2 now cannot be used
    println!("mutable r3 = {}", r3);

    let r3 = &s2; // mutable ref shadowed to immutable ref
    println!("immutable r3 = {}", r3);
            
}

// We point the the initial s, however we cannot mutate the given reference
fn calculate_only_length(s: &String) -> usize {
    let length = s.len();
    length
}

// This function points to the initals
fn change_string(s: &mut String) {
    s.push_str(" changed_string");
}

// NOTE: Slicing must be done via a reference
fn slice_type() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // immutable reference (should be value 5)
    s.clear(); // string is now ""

    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello = &s[..5]; // equal to above statement
    let world = &s[6..s.len()];
    let world = &s[6..]; // equal to above statement

    println!("{} {}", hello, world);

    // Func works for string obj
    let s = String::from("hello world");
    let word = first_word(&s);
    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);

    // Func works for string literal
    let s = "hello world";
    let word = first_word(&s);
    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("Slicing assertion passed!");
}

// Sense we are returning a ref, if the primary string deletes, so does
// the returned content of this function.
// NOTE: &str can be used for immutable string ref, however str can only exist
// in reference format.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // Iterate through each character
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    //&s
    &s[..] // equal to the above statement
}
