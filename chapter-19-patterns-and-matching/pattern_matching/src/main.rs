pub mod pattern_usage;
pub mod refutability;
pub mod pattern_syntax;

fn main() {
    pattern_usage::run();
    println!("+++++++++++++++++++");
    refutability::run();
    println!("+++++++++++++++++++");
    pattern_syntax::run();
}
