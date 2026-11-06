pub mod oop;
pub mod shared_behavior;
pub mod design_pattern_implementation;

fn main() {
    oop::run();
    println!("+++++++++++++++++++");
    shared_behavior::run();
    println!("+++++++++++++++++++");
    design_pattern_implementation::run();
}
