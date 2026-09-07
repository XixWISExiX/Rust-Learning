// Constants can be declared anyware, including globally.
// However constants must be annotated with a Data Type (here it's u32).
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of the const is: {THREE_HOURS_IN_SECONDS}");
    
    // Immutable variable.
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // Errors out here
    // println!("The value of x is: {x}");

    // Mutable variable.
    let mut y = 6;
    println!("The value of mutable y is: {y}");
    y = 7;
    println!("The value of mutable y is: {y}");

    // Shadowing, allows you to create a new variable with the same name as
    // the previous old variable.
    let z = 10;
    let z = z + 1; // shadowed x value

    {
        let z = z * 2;
        println!("The value of z in this inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // ------------------------
    // SCALAR types
    // ------------------------
    let bit8 : i8 = -8;
    let bit16 : i16 = -16;
    let bit32 : i32 = -32;
    let bit64 : i64 = -64;
    let bit128 : i128 = -128;

    let unsigned_bit8 : u8 = 8;
    let unsigned_bit16 : u16 = 16;
    let unsigned_bit32 : u32 = 32;
    let unsigned_bit64 : u64 = 64;
    let unsigned_bit128 : u128 = 128;

    let a = 2.0; // f64 (double)
    let b: f64 = 2.0; // f64 (double)
    let c: f32 = 3.0; // f32 (float)

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Characters (char is 4-bytes in Rust)
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ------------------------
    // COMPOUND TYPES
    // ------------------------

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (e, f, g) = tup;
    let h = tup.0;
    let i = tup.1;
    let j = tup.2;
    println!("Tuple result through var: e={e}, f={f}, g={g}");
    println!("Tuple result through indexing: h={h}, i={i}, j={j}");

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let arr_2 : [i32; 3] = [1, 2, 3];
    let arr_3 = [0; 5]; // Generates this arr [0, 0, 0, 0, 0]
    println!("arr elements: {}, {}", arr[0], arr[1]);
    // NOTE: During run time, if the user access a value that's outside the array size, the
    // the program shuts down. This is Rust's memory management in action.

    println!("\nNOTE: lots of the data types are in the programming file and not printed out.");
}
