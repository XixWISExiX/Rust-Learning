pub mod generic_types;
pub mod traits;
pub mod lifetimes;

fn main() {
    generic_types::run();
    println!("++++++++++++++++++++");
    traits::run();
    println!("++++++++++++++++++++");
    lifetimes::run();
}
