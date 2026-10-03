pub mod pointer_box;
pub mod pointer_reference_counter;
pub mod interior_mutability_pattern;
pub mod memory_leaks;

fn main() {
    pointer_box::run();
    println!("+++++++++++++++++++");
    pointer_reference_counter::run();
    println!("+++++++++++++++++++");
    interior_mutability_pattern::run();
    println!("+++++++++++++++++++");
    memory_leaks::run();
}
