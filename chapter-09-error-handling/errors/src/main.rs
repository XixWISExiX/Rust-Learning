pub mod unrecoverable_errors;
pub mod recoverable_errors;
pub mod when_to_panic;

fn main() {
    unrecoverable_errors::run();
    println!("++++++++++++++++++++");
    recoverable_errors::run();
    println!("++++++++++++++++++++");
    when_to_panic::run();
}
