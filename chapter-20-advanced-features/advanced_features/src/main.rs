pub mod unsafe_rust;
pub mod traits;
pub mod types;
pub mod functions_and_closures;
pub mod macros;

fn main() {
    unsafe_rust::run();
    println!("+++++++++++++++++++");
    traits::run();
    println!("+++++++++++++++++++");
    types::run();
    println!("+++++++++++++++++++");
    functions_and_closures::run();
    println!("+++++++++++++++++++");
    macros::run();
}

