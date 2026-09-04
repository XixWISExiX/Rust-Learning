pub mod unrecoverable_errors;
pub mod recoverable_errors;
pub mod panic;

fn main() {
    unrecoverable_errors::run();
    println!("++++++++++++++++++++");
    recoverable_errors::run();
    println!("++++++++++++++++++++");
    panic::run();
}
